use std::str::FromStr;

use async_trait::async_trait;
use repaint_server_model::event::Event;
use repaint_server_model::id::Id;
use repaint_server_model::visitor_image::CurrentImage;
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
}

#[derive(Debug)]
pub struct VisitorUsecaseImpl<R, F> {
    repo: R,
    firestore: F,
}

#[inject]
impl<R, F> VisitorUsecaseImpl<R, F>
where
    R: VisitorRepository + EventRepository + SpotRepository + ImageRepository + PaletteRepository,
    F: Firestore,
{
    pub fn new(repo: R, firestore: F) -> Self {
        Self { repo, firestore }
    }
}

#[async_trait]
impl<R, F> VisitorUsecase for VisitorUsecaseImpl<R, F>
where
    R: VisitorRepository + EventRepository + SpotRepository + ImageRepository + PaletteRepository,
    F: Firestore,
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
        let spots = SpotRepository::list(&self.repo, event.id).await?;
        let images = ImageRepository::list_default_image(&self.repo, event.id).await?;
        let visitor =
            VisitorRepository::create(&self.repo, event.id, registration_id.clone()).await?;
        let palettes = PaletteRepository::get(&self.repo, visitor.id).await?;

        self.firestore
            .subscribe_register_log(event_id, visitor.visitor_id)
            .await?;

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
        let spots = SpotRepository::list(&self.repo, event.id).await?;
        let images = ImageRepository::list_default_image(&self.repo, event.id).await?;
        let visitor =
            VisitorRepository::get(&self.repo, event.id, visitor_identification.visitor_id)
                .await?
                .ok_or(Error::BadRequest {
                    message: format!("{} is invalid id", visitor_identification.visitor_id),
                })?;
        let palettes = PaletteRepository::get(&self.repo, visitor.id).await?;
        let image_id = ImageRepository::get_visitor_image(&self.repo, visitor.id).await?;
        let current_image_id = match ImageRepository::get_current_image(&self.repo, visitor.id)
            .await?
        {
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

        self.firestore
            .subscribe_initialize_log(
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
        let _ = VisitorRepository::delete(&self.repo, visitor.id).await?;

        Ok(())
    }
}
