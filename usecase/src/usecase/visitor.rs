use std::str::FromStr;

use async_trait::async_trait;
use futures::future::join_all;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use repaint_server_model::event::Event;
use repaint_server_model::event_spot::EventSpot;
use repaint_server_model::id::Id;
use repaint_server_model::visitor_image::{CurrentImage, Image as VisitorImage};
use repaint_server_model::AsyncSafe;
use teloc::inject;

use crate::infra::firestore::Firestore;
use crate::infra::repo::{
    EventRepository, ImageRepository, PaletteRepository, SpotRepository, VisitorRepository,
};
use crate::model::event::EventResponse;
use crate::model::visitor::{RegisterVisitorResponse, VisitorIdentification, VisitorResponse};
use crate::usecase::error::Error;

#[async_trait]
pub trait VisitorUsecase: AsyncSafe {
    async fn join_event(
        &self,
        event_id: Id<Event>,
        registration_id: String,
    ) -> Result<(EventResponse, RegisterVisitorResponse), Error>;

    async fn initialize_visitor(
        &self,
        visitor_identification: VisitorIdentification,
        registration_id: String,
    ) -> Result<(EventResponse, VisitorResponse), Error>;

    async fn delete_visitor(
        &self,
        visitor_identification: VisitorIdentification,
    ) -> Result<(), Error>;

    async fn list_image(
        &self,
        visitor_identification: VisitorIdentification,
    ) -> Result<Vec<Id<VisitorImage>>, Error>;

    async fn get_current_image(
        &self,
        visitor_identification: VisitorIdentification,
    ) -> Result<Id<CurrentImage>, Error>;

    async fn set_current_image(
        &self,
        visitor_identification: VisitorIdentification,
        image_id: Id<VisitorImage>,
    ) -> Result<(), Error>;

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
pub struct VisitorUsecaseImpl<R> {
    repo: R,
}

#[inject]
impl<R> VisitorUsecaseImpl<R>
where
    R: VisitorRepository
        + EventRepository
        + SpotRepository
        + ImageRepository
        + PaletteRepository
        + Firestore,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R> VisitorUsecase for VisitorUsecaseImpl<R>
where
    R: VisitorRepository
        + EventRepository
        + SpotRepository
        + ImageRepository
        + PaletteRepository
        + Firestore,
{
    async fn join_event(
        &self,
        event_id: Id<Event>,
        registration_id: String,
    ) -> Result<(EventResponse, RegisterVisitorResponse), Error> {
        if registration_id.chars().count() > 4096 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 4096 chars", registration_id),
            });
        }

        let event = EventRepository::get(&self.repo, event_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", event_id),
            })?;
        let spots = SpotRepository::list_with_event_id(&self.repo, event.event_id).await?;
        let images = ImageRepository::list_default_image(&self.repo, event.event_id).await?;
        let visitor =
            VisitorRepository::create(&self.repo, event_id, registration_id.clone()).await?;
        let palettes = PaletteRepository::get(&self.repo, visitor.visitor_id).await?;

        Firestore::subscribe_register_log(&self.repo, event_id, visitor.visitor_id).await?;

        Ok((
            EventResponse {
                event_id,
                name: event.name,
                hp_url: event.hp_url,
                contact: event.contact,
                spots,
                images,
            },
            RegisterVisitorResponse {
                visitor_identification: VisitorIdentification {
                    event_id,
                    visitor_id: visitor.visitor_id,
                },
                registration_id,
                palettes,
            },
        ))
    }

    async fn initialize_visitor(
        &self,
        visitor_identification: VisitorIdentification,
        registration_id: String,
    ) -> Result<(EventResponse, VisitorResponse), Error> {
        if registration_id.chars().count() > 4096 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 4096 chars", registration_id),
            });
        }

        let event = EventRepository::get(&self.repo, visitor_identification.event_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", visitor_identification.event_id),
            })?;
        let spots = SpotRepository::list_with_event_id(&self.repo, event.event_id).await?;
        let images = ImageRepository::list_default_image(&self.repo, event.event_id).await?;
        let visitor = VisitorRepository::get(&self.repo, visitor_identification.clone())
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", visitor_identification.visitor_id),
            })?;
        let palettes = PaletteRepository::get(&self.repo, visitor.visitor_id).await?;
        let image_id = ImageRepository::get_visitor_image(&self.repo, visitor.visitor_id).await?;
        let current_image_id =
            ImageRepository::get_current_image(&self.repo, visitor.visitor_id).await?;

        Firestore::subscribe_initialize_log(
            &self.repo,
            visitor_identification.event_id,
            visitor_identification.visitor_id,
        )
        .await?;

        Ok((
            EventResponse {
                event_id: visitor_identification.event_id,
                name: event.name,
                hp_url: event.hp_url,
                contact: event.contact,
                spots,
                images,
            },
            VisitorResponse {
                visitor_identification,
                registration_id,
                palettes,
                image_id,
                current_image_id,
            },
        ))
    }

    async fn delete_visitor(
        &self,
        visitor_identification: VisitorIdentification,
    ) -> Result<(), Error> {
        let _ = VisitorRepository::delete(&self.repo, visitor_identification).await?;

        Ok(())
    }

    async fn list_image(
        &self,
        visitor_identification: VisitorIdentification,
    ) -> Result<Vec<Id<VisitorImage>>, Error> {
        let default =
            ImageRepository::list_default_image(&self.repo, visitor_identification.event_id)
                .await?;
        let visitor =
            ImageRepository::get_visitor_image(&self.repo, visitor_identification.visitor_id)
                .await?;
        let mut images = default
            .iter()
            .filter_map(|&i| Id::<VisitorImage>::from_str(&i.to_string()).ok())
            .collect::<Vec<_>>();
        if let Some(visitor) = visitor {
            images.push(visitor);
        };

        Ok(images)
    }

    async fn get_current_image(
        &self,
        visitor_identification: VisitorIdentification,
    ) -> Result<Id<CurrentImage>, Error> {
        let current_image_id =
            ImageRepository::get_current_image(&self.repo, visitor_identification.visitor_id)
                .await?;

        Ok(current_image_id)
    }

    async fn set_current_image(
        &self,
        visitor_identification: VisitorIdentification,
        image_id: Id<VisitorImage>,
    ) -> Result<(), Error> {
        let _ = ImageRepository::set_current_image(
            &self.repo,
            visitor_identification.visitor_id,
            image_id,
        )
        .await?;

        Ok(())
    }

    async fn drop_palette(
        &self,
        visitor_identification: VisitorIdentification,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Error> {
        let palettes =
            PaletteRepository::get(&self.repo, visitor_identification.visitor_id).await?;
        let took_photo =
            ImageRepository::get_visitor_image(&self.repo, visitor_identification.visitor_id)
                .await?
                .is_some();
        let is_bonus =
            SpotRepository::get_bonus_state(&self.repo, visitor_identification.event_id, spot_id)
                .await?;

        Firestore::subscribe_visitor_log(
            &self.repo,
            visitor_identification.event_id,
            visitor_identification.visitor_id,
            spot_id,
            palettes.len(),
            took_photo,
        )
        .await?;
        Firestore::subscribe_visitor(
            &self.repo,
            visitor_identification.event_id,
            visitor_identification.visitor_id,
            spot_id,
        )
        .await?;

        let mut rng = {
            let rng = rand::thread_rng();
            StdRng::from_rng(rng).unwrap()
        };

        if is_bonus {
            let palettes =
                Firestore::get_palettes(&self.repo, visitor_identification.event_id, spot_id)
                    .await?;
            let palettes = palettes
                .choose_multiple(&mut rng, 2)
                .cloned()
                .collect::<Vec<_>>();
            let p = palettes
                .iter()
                .map(|&p| PaletteRepository::set(&self.repo, visitor_identification.visitor_id, p));
            join_all(p)
                .await
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?;

            let palettes = palettes.choose_multiple(&mut rng, 2).cloned().collect();
            Firestore::subscribe_palettes(
                &self.repo,
                visitor_identification.event_id,
                spot_id,
                palettes,
            )
            .await?;
        } else {
            let palette =
                Firestore::get_palette(&self.repo, visitor_identification.event_id, spot_id)
                    .await?;
            let _ = PaletteRepository::set(&self.repo, visitor_identification.visitor_id, palette)
                .await?;

            let palette = palettes.choose(&mut rng).cloned().unwrap();
            Firestore::subscribe_palette(
                &self.repo,
                visitor_identification.event_id,
                spot_id,
                palette,
            )
            .await?;
        }

        Ok(())
    }

    async fn pick_palette(
        &self,
        visitor_identification: VisitorIdentification,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Error> {
        let visitors =
            VisitorRepository::list_with_event_id(&self.repo, visitor_identification.event_id)
                .await?;

        let p = visitors
            .iter()
            .map(|v| PaletteRepository::get(&self.repo, v.visitor_id));

        let palettes = join_all(p)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        let visitor_palettes =
            PaletteRepository::get(&self.repo, visitor_identification.visitor_id)
                .await?
                .into_iter()
                .collect::<Vec<_>>();

        let mut rng = {
            let rng = rand::thread_rng();
            StdRng::from_rng(rng).unwrap()
        };

        while let Some(&palette) = palettes.choose(&mut rng) {
            if !visitor_palettes.contains(&palette) {
                let _ =
                    PaletteRepository::set(&self.repo, visitor_identification.visitor_id, palette);
                break;
            }
        }

        Ok(())
    }
}
