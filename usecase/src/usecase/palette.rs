use std::str::FromStr;

use async_trait::async_trait;
use futures::future::join_all;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use repaint_server_model::event_spot::EventSpot;
use repaint_server_model::id::Id;
use repaint_server_model::visitor_image::{CurrentImage, Image as VisitorImage};
use repaint_server_model::AsyncSafe;
use teloc::inject;

use crate::infra::firestore::Firestore;
use crate::infra::pubsub::GoogleCloudPubSub;
use crate::infra::repo::{
    EventRepository, ImageRepository, PaletteRepository, SpotRepository, VisitorRepository,
};
use crate::model::visitor::VisitorIdentification;
use crate::usecase::error::Error;

#[async_trait]
pub trait PaletteUsecase: AsyncSafe {
    async fn drop_palette(
        &self,
        visitor_identification: VisitorIdentification,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Error>;

    async fn pick_palette(
        &self,
        visitor_identification: VisitorIdentification,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct PaletteUsecaseImpl<R, F, P> {
    repo: R,
    firestore: F,
    pubsub: P,
}

#[inject]
impl<R, F, P> PaletteUsecaseImpl<R, F, P>
where
    R: PaletteRepository + EventRepository + VisitorRepository + ImageRepository + SpotRepository,
    F: Firestore,
    P: GoogleCloudPubSub,
{
    pub fn new(repo: R, firestore: F, pubsub: P) -> Self {
        Self {
            repo,
            firestore,
            pubsub,
        }
    }
}

#[async_trait]
impl<R, F, P> PaletteUsecase for PaletteUsecaseImpl<R, F, P>
where
    R: PaletteRepository + EventRepository + VisitorRepository + ImageRepository + SpotRepository,
    F: Firestore,
    P: GoogleCloudPubSub,
{
    async fn drop_palette(
        &self,
        visitor_identification: VisitorIdentification,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Error> {
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
        let palettes = PaletteRepository::get(&self.repo, visitor.id).await?;
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

                Id::<CurrentImage>::from_str(&current_image_id.to_string())
                    .ok()
                    .ok_or(Error::BadRequest {
                        message: "failed to parse default image id to current image id".to_string(),
                    })?
            }
        };
        let image_id = Id::<VisitorImage>::from_str(&image.to_string())?;
        let _ = self
            .pubsub
            .publish_merge_current_image(
                event.event_id,
                visitor.visitor_id,
                image_id,
                palettes.clone(),
            )
            .await?;
        let took_photo = ImageRepository::get_visitor_image(&self.repo, visitor.id)
            .await?
            .is_some();
        let is_bonus = SpotRepository::get_bonus_state(&self.repo, event.id, spot_id).await?;

        self.firestore
            .subscribe_visitor_log(
                visitor_identification.event_id,
                visitor_identification.visitor_id,
                spot_id,
                palettes.len(),
                took_photo,
            )
            .await?;
        self.firestore
            .subscribe_visitor(
                visitor_identification.event_id,
                visitor_identification.visitor_id,
                spot_id,
            )
            .await?;

        let mut rng = {
            let rng = rand::thread_rng();
            StdRng::from_rng(rng).unwrap()
        };
        let visitors = VisitorRepository::list(&self.repo, event.id).await?;

        let p = visitors
            .iter()
            .map(|v| PaletteRepository::get(&self.repo, v.id));
        let palettes = join_all(p)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        if is_bonus {
            let palettes = self
                .firestore
                .get_palettes(visitor_identification.event_id, spot_id)
                .await?
                .unwrap_or(palettes);
            let palettes = palettes
                .choose_multiple(&mut rng, 2)
                .cloned()
                .collect::<Vec<_>>();
            let p = palettes
                .iter()
                .map(|&p| PaletteRepository::set(&self.repo, visitor.id, p));
            join_all(p)
                .await
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?;

            let palettes = palettes.choose_multiple(&mut rng, 2).cloned().collect();
            self.firestore
                .subscribe_palettes(visitor_identification.event_id, spot_id, palettes)
                .await?;
        } else {
            let palette = self
                .firestore
                .get_palette(visitor_identification.event_id, spot_id)
                .await?
                .unwrap_or(palettes.choose(&mut rng).cloned().unwrap());
            let _ = PaletteRepository::set(&self.repo, visitor.id, palette).await?;

            let palette = palettes.choose(&mut rng).cloned().unwrap();
            self.firestore
                .subscribe_palette(visitor_identification.event_id, spot_id, palette)
                .await?;
        }

        Ok(())
    }

    async fn pick_palette(
        &self,
        visitor_identification: VisitorIdentification,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Error> {
        let event = EventRepository::get(&self.repo, visitor_identification.event_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", visitor_identification.event_id),
            })?;
        let Some(_) = SpotRepository::get_by_qr(&self.repo, event.id, spot_id).await? else {
            return Err(Error::BadRequest {
                message: format!("{} is invalid id", spot_id),
            });
        };
        let visitors = VisitorRepository::list(&self.repo, event.id).await?;

        let p = visitors
            .iter()
            .map(|v| PaletteRepository::get(&self.repo, v.id));
        let palettes = join_all(p)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

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

                Id::<CurrentImage>::from_str(&current_image_id.to_string())
                    .ok()
                    .ok_or(Error::BadRequest {
                        message: "failed to parse default image id to current image id".to_string(),
                    })?
            }
        };
        let image_id = Id::<VisitorImage>::from_str(&image.to_string())?;
        let _ = self
            .pubsub
            .publish_merge_current_image(
                event.event_id,
                visitor.visitor_id,
                image_id,
                visitor_palettes.clone(),
            )
            .await?;

        let mut rng = {
            let rng = rand::thread_rng();
            StdRng::from_rng(rng).unwrap()
        };

        while let Some(&palette) = palettes.choose(&mut rng) {
            if !visitor_palettes.contains(&palette) {
                let _ = PaletteRepository::set(&self.repo, visitor.id, palette);
                break;
            }
        }

        Ok(())
    }
}
