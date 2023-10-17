use std::str::FromStr;

use async_trait::async_trait;
use chrono::{Duration, Utc};
use itertools::Itertools;
use repaint_server_model::event_spot::EventSpot;
use repaint_server_model::id::Id;
use repaint_server_model::visitor_image::Image as VisitorImage;
use repaint_server_model::AsyncSafe;
use repaint_server_util::envvar;
use teloc::inject;

use crate::infra::pubsub::GoogleCloudPubSub;
use crate::infra::repo::{
    EventRepository, ImageRepository, PaletteRepository, SpotRepository, TransactionRepository,
    VisitorRepository,
};
use crate::model::palette::CheckPalettesCompletedResponse;
use crate::model::visitor::VisitorIdentification;
use crate::usecase::error::Error;

#[async_trait]
pub trait PaletteUsecase: AsyncSafe {
    async fn pick_palette(
        &self,
        visitor_identification: VisitorIdentification,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Error>;

    async fn check_palettes_completed(
        &self,
        visitor_identification: VisitorIdentification,
    ) -> Result<CheckPalettesCompletedResponse, Error>;
}

#[derive(Debug)]
pub struct PaletteUsecaseImpl<R, P> {
    repo: R,
    pubsub: P,
}

#[inject]
impl<R, P> PaletteUsecaseImpl<R, P>
where
    R: PaletteRepository
        + EventRepository
        + VisitorRepository
        + ImageRepository
        + SpotRepository
        + TransactionRepository,
    P: GoogleCloudPubSub,
{
    pub fn new(repo: R, pubsub: P) -> Self {
        Self { repo, pubsub }
    }
}

#[async_trait]
impl<R, P> PaletteUsecase for PaletteUsecaseImpl<R, P>
where
    R: PaletteRepository
        + EventRepository
        + VisitorRepository
        + ImageRepository
        + SpotRepository
        + TransactionRepository,
    P: GoogleCloudPubSub,
{
    async fn pick_palette(
        &self,
        visitor_identification: VisitorIdentification,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Error> {
        let now = Utc::now().naive_utc();
        let tx = TransactionRepository::begin_transaction(&self.repo).await?;
        let event = EventRepository::get(&self.repo, &tx, visitor_identification.event_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", visitor_identification.event_id),
            })?;
        let visitor =
            VisitorRepository::get(&self.repo, &tx, event.id, visitor_identification.visitor_id)
                .await?
                .ok_or(Error::BadRequest {
                    message: format!("{} is invalid id", visitor_identification.visitor_id),
                })?;
        let spot = SpotRepository::get_by_qr(&self.repo, &tx, event.id, spot_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", spot_id),
            })?;
        let visitor_palettes = PaletteRepository::get(&self.repo, visitor.id)
            .await?
            .into_iter()
            .collect::<Vec<_>>();
        if visitor_palettes.len() == envvar("CLUSTER", None) {
            return Err(Error::RangeNotSatisfiable);
        }
        let last_picked =
            VisitorRepository::get_last_picked_at(&self.repo, &tx, visitor.id, spot.id).await?;
        let last_scaned =
            VisitorRepository::get_last_scanned_at(&self.repo, &tx, visitor.id, spot.id).await?;
        let is_bonus =
            SpotRepository::get_bonus_state(&self.repo, &tx, event.id, spot.spot_id).await?;
        if (last_scaned.is_some()
            && now - last_scaned.unwrap() >= Duration::seconds(envvar("VISITOR_SPOT_TIMEOUT", 300)))
            || (last_picked.is_some()
                && now - last_picked.unwrap()
                    <= Duration::seconds(if is_bonus {
                        envvar("BONUS_PICK_INTERVAL", 180)
                    } else {
                        envvar("PICK_INTERVAL", 300)
                    }))
        {
            return Err(Error::Conflict);
        }
        let Some(mut palettes) = PaletteRepository::get_all(&self.repo, &tx, event.id).await?
        else {
            unreachable!("palettes is not set")
        };
        let visitor_palettes = PaletteRepository::get(&self.repo, &tx, visitor.id)
            .await?
            .into_iter()
            .collect::<Vec<_>>();
        let image_id = match ImageRepository::get_current_image(&self.repo, &tx, visitor.id).await?
        {
            Some(i) => Id::<VisitorImage>::from_str(i.to_string().as_str())?,
            None => {
                let default =
                    ImageRepository::list_default_image(&self.repo, &tx, event.id).await?;
                let event_image_id = default
                    .first()
                    .ok_or(Error::BadRequest {
                        message: "default image is empty".to_string(),
                    })?
                    .clone();

                Id::<VisitorImage>::from_str(event_image_id.to_string().as_str())?
            }
        };
        let _ = self
            .pubsub
            .publish_merge_current_image(
                event.event_id,
                visitor.visitor_id,
                image_id,
                visitor_palettes.clone(),
            )
            .await?;
        let cloned_palettes = palettes.clone();
        let sorted_palettes = cloned_palettes
            .iter()
            .enumerate()
            .sorted_by_key(|(_, &p)| p)
            .map(|(i, _)| i as i32)
            .collect::<Vec<_>>();

        let mut i = 0;
        loop {
            let palette = sorted_palettes[i];
            if !visitor_palettes.contains(&palette) {
                palettes[palette as usize] += 1;
                let _ = PaletteRepository::set(&self.repo, &tx, visitor.id, palette).await?;
                let _ = PaletteRepository::set_all(&self.repo, &tx, event.id, palettes).await?;
                break;
            } else if palettes
                .iter()
                .all(|palette| visitor_palettes.contains(palette))
            {
                break;
            } else if i == envvar::<usize, _>("CLUSTER", None) {
                break;
            }
            i += 1;
        }
        let _ = VisitorRepository::set_last_picked_at(&self.repo, &tx, visitor.id, spot.id, now)
            .await?;
        let _ = tx.commit().await?;

        Ok(())
    }

    async fn check_palettes_completed(
        &self,
        visitor_identification: VisitorIdentification,
    ) -> Result<CheckPalettesCompletedResponse, Error> {
        let tx = TransactionRepository::begin_transaction(&self.repo).await?;
        let event = EventRepository::get(&self.repo, &tx, visitor_identification.event_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", visitor_identification.event_id),
            })?;
        let visitor =
            VisitorRepository::get(&self.repo, &tx, event.id, visitor_identification.visitor_id)
                .await?
                .ok_or(Error::BadRequest {
                    message: format!("{} is invalid id", visitor_identification.visitor_id),
                })?;
        let visitor_palettes = PaletteRepository::get(&self.repo, &tx, visitor.id)
            .await?
            .into_iter()
            .collect::<Vec<_>>();
        let _ = tx.commit().await?;

        Ok(CheckPalettesCompletedResponse {
            is_completed: visitor_palettes.len() == envvar::<usize, _>("CLUSTER", None),
        })
    }
}
