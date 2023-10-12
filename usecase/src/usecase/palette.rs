use std::str::FromStr;

use async_trait::async_trait;
use chrono::{Duration, Utc};
use repaint_server_model::event_spot::EventSpot;
use repaint_server_model::id::Id;
use repaint_server_model::visitor_image::{CurrentImage, Image as VisitorImage};
use repaint_server_model::AsyncSafe;
use repaint_server_util::envvar;
use teloc::inject;

use crate::infra::pubsub::GoogleCloudPubSub;
use crate::infra::repo::{
    EventRepository, ImageRepository, PaletteRepository, SpotRepository, VisitorRepository,
};
use crate::model::visitor::VisitorIdentification;
use crate::usecase::error::Error;

#[async_trait]
pub trait PaletteUsecase: AsyncSafe {
    async fn pick_palette(
        &self,
        visitor_identification: VisitorIdentification,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct PaletteUsecaseImpl<R, P> {
    repo: R,
    pubsub: P,
}

#[inject]
impl<R, P> PaletteUsecaseImpl<R, P>
where
    R: PaletteRepository + EventRepository + VisitorRepository + ImageRepository + SpotRepository,
    P: GoogleCloudPubSub,
{
    pub fn new(repo: R, pubsub: P) -> Self {
        Self { repo, pubsub }
    }
}

#[async_trait]
impl<R, P> PaletteUsecase for PaletteUsecaseImpl<R, P>
where
    R: PaletteRepository + EventRepository + VisitorRepository + ImageRepository + SpotRepository,
    P: GoogleCloudPubSub,
{
    async fn pick_palette(
        &self,
        visitor_identification: VisitorIdentification,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Error> {
        let now = Utc::now().naive_utc();
        let event = EventRepository::get(&self.repo, visitor_identification.event_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", visitor_identification.event_id),
            })?;
        let visitor =
            VisitorRepository::get(&self.repo, event.id, visitor_identification.visitor_id)
                .await?
                .ok_or(Error::BadRequest {
                    message: format!("{} is invalid id", visitor_identification.visitor_id),
                })?;
        let spot = SpotRepository::get_by_qr(&self.repo, event.id, spot_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", spot_id),
            })?;
        let last_picked =
            VisitorRepository::get_last_picked_at(&self.repo, visitor.id, spot.id).await?;
        let last_scaned =
            VisitorRepository::get_last_scanned_at(&self.repo, visitor.id, spot.id).await?;
        let is_bonus = SpotRepository::get_bonus_state(&self.repo, event.id, spot.spot_id).await?;
        if (last_scaned.is_some()
            && now - last_scaned.unwrap() <= Duration::seconds(envvar("VISITOR_SPOT_TIMEOUT", 300)))
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
        let Some(mut palettes) = PaletteRepository::get_all(&self.repo, event.id).await? else {
            unreachable!("palettes is not set")
        };
        let visitor_palettes = PaletteRepository::get(&self.repo, visitor.id)
            .await?
            .into_iter()
            .collect::<Vec<_>>();
        let image = match ImageRepository::get_current_image(&self.repo, visitor.id).await? {
            Some(i) => i,
            None => {
                let default = ImageRepository::list_default_image(&self.repo, event.id).await?;
                let current_image_id = default
                    .first()
                    .ok_or(Error::BadRequest {
                        message: "default image is empty".to_string(),
                    })?
                    .clone();
                Id::<CurrentImage>::from_str(current_image_id.to_string().as_str())
                    .ok()
                    .ok_or(Error::BadRequest {
                        message: "failed to parse default image id to current image id".to_string(),
                    })?
            }
        };
        let image_id = Id::<VisitorImage>::from_str(image.to_string().as_str())?;
        let _ = self
            .pubsub
            .publish_merge_current_image(
                event.event_id,
                visitor.visitor_id,
                image_id,
                visitor_palettes.clone(),
            )
            .await?;
        loop {
            let palette = match palettes.iter().enumerate().min_by_key(|(_, &v)| v) {
                Some((i, _)) => {
                    palettes[i] += 1;
                    let _ =
                        PaletteRepository::set_all(&self.repo, event.id, palettes.clone()).await?;

                    i as i32
                }
                None => unreachable!("palettes is empty"),
            };
            if !visitor_palettes.contains(&palette) {
                let _ = PaletteRepository::set(&self.repo, visitor.id, palette);
                break;
            } else if palettes
                .iter()
                .all(|&palette| visitor_palettes.contains(&palette))
            {
                break;
            }
        }

        Ok(())
    }
}
