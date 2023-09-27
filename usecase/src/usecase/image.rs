use std::str::FromStr;

use async_trait::async_trait;
use repaint_server_model::event::Event;
use repaint_server_model::event_image::Image as EventImage;
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_model::visitor_image::{CurrentImage, Image as VisitorImage};
use repaint_server_model::AsyncSafe;
use teloc::inject;

use crate::infra::gcs::GoogleCloudStorage;
use crate::infra::otp::ImageOtp;
use crate::infra::pubsub::GoogleCloudPubSub;
use crate::infra::repo::{EventRepository, ImageRepository, PaletteRepository, VisitorRepository};
use crate::model::visitor::VisitorIdentification;
use crate::usecase::error::Error;

#[async_trait]
pub trait ImageUsecase: AsyncSafe {
    async fn add_default_image(
        &self,
        subject: String,
        event_id: Id<Event>,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    async fn delete_default_image(
        &self,
        subject: String,
        event_id: Id<Event>,
        image_id: Id<EventImage>,
    ) -> Result<(), Error>;

    async fn check_visitor_image_exist(
        &self,
        subject: String,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
    ) -> Result<Option<Id<VisitorImage>>, Error>;

    async fn upload_visitor_image(
        &self,
        subject: String,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
        data: Vec<u8>,
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

    async fn proxy_image(
        &self,
        event_id: Id<Event>,
        image_id: Id<CurrentImage>,
        visitor_id: Id<Visitor>,
    ) -> Result<String, Error>;

    async fn set_update(&self, event_id: Id<Event>, visitor_id: Id<Visitor>) -> Result<(), Error>;

    async fn check_update(
        &self,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
    ) -> Result<bool, Error>;
}

#[derive(Debug, Clone)]
pub struct ImageUsecaseImpl<R, S, O, P> {
    repo: R,
    storage: S,
    otp: O,
    pubsub: P,
}

#[inject]
impl<R, S, O, P> ImageUsecaseImpl<R, S, O, P>
where
    R: ImageRepository + EventRepository + VisitorRepository + PaletteRepository,
    S: GoogleCloudStorage,
    O: ImageOtp,
    P: GoogleCloudPubSub,
{
    pub fn new(repo: R, storage: S, otp: O, pubsub: P) -> Self {
        Self {
            repo,
            storage,
            otp,
            pubsub,
        }
    }
}

#[async_trait]
impl<R, S, O, P> ImageUsecase for ImageUsecaseImpl<R, S, O, P>
where
    R: ImageRepository + EventRepository + VisitorRepository + PaletteRepository,
    S: GoogleCloudStorage,
    O: ImageOtp,
    P: GoogleCloudPubSub,
{
    async fn add_default_image(
        &self,
        subject: String,
        event_id: Id<Event>,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        let event = EventRepository::get_event_belong_to_subject(&self.repo, subject, event_id)
            .await?
            .ok_or(Error::UnAuthorized)?;
        let image_id = Id::<EventImage>::new();
        let _ = self
            .storage
            .upload_event_image(data, event_id, image_id)
            .await?;
        let _ = self
            .pubsub
            .publish_clustering_event_image(event.event_id, image_id)
            .await?;
        let _ = ImageRepository::add_default_image(&self.repo, event.id, image_id).await?;

        Ok(())
    }

    async fn delete_default_image(
        &self,
        subject: String,
        event_id: Id<Event>,
        image_id: Id<EventImage>,
    ) -> Result<(), Error> {
        let event = EventRepository::get_event_belong_to_subject(&self.repo, subject, event_id)
            .await?
            .ok_or(Error::UnAuthorized)?;
        let _ = ImageRepository::delete_default_image(&self.repo, event.id, image_id).await?;

        Ok(())
    }

    async fn check_visitor_image_exist(
        &self,
        subject: String,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
    ) -> Result<Option<Id<VisitorImage>>, Error> {
        let event = EventRepository::get_event_belong_to_subject(&self.repo, subject, event_id)
            .await?
            .ok_or(Error::UnAuthorized)?;
        let visitor = VisitorRepository::get(&self.repo, event.id, visitor_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} aren't exist", visitor_id),
            })?;
        let image = ImageRepository::get_visitor_image(&self.repo, visitor.id).await?;

        Ok(image)
    }

    async fn upload_visitor_image(
        &self,
        subject: String,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        let event = EventRepository::get_event_belong_to_subject(&self.repo, subject, event_id)
            .await?
            .ok_or(Error::UnAuthorized)?;
        let visitor = VisitorRepository::get(&self.repo, event.id, visitor_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} aren't exist", visitor_id),
            })?;
        let image_id = Id::<VisitorImage>::new();
        let _ = self
            .storage
            .upload_visitor_image(data, event_id, image_id)
            .await?;
        let _ = self
            .pubsub
            .publish_clustering_visitor_image(event.event_id, visitor.visitor_id, image_id)
            .await?;
        let _ = ImageRepository::upload_visitor_image(&self.repo, visitor.id, image_id).await?;

        Ok(())
    }

    async fn list_image(
        &self,
        visitor_identification: VisitorIdentification,
    ) -> Result<Vec<Id<VisitorImage>>, Error> {
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
        let default = ImageRepository::list_default_image(&self.repo, event.id).await?;
        let visitor = ImageRepository::get_visitor_image(&self.repo, visitor.id).await?;
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

        Ok(current_image_id)
    }

    async fn set_current_image(
        &self,
        visitor_identification: VisitorIdentification,
        image_id: Id<VisitorImage>,
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
        let _ = ImageRepository::set_current_image(&self.repo, visitor.id, image_id).await?;
        let _ = self
            .pubsub
            .publish_merge_current_image(event.event_id, visitor.visitor_id, image_id, palettes)
            .await?;

        Ok(())
    }

    async fn proxy_image(
        &self,
        event_id: Id<Event>,
        image_id: Id<CurrentImage>,
        visitor_id: Id<Visitor>,
    ) -> Result<String, Error> {
        let token = self.otp.verify(event_id, image_id, visitor_id).await?;

        Ok(token.full)
    }

    async fn set_update(&self, event_id: Id<Event>, visitor_id: Id<Visitor>) -> Result<(), Error> {
        let event = EventRepository::get(&self.repo, event_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", event_id),
            })?;
        let visitor = VisitorRepository::get(&self.repo, event.id, visitor_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", visitor_id),
            })?;

        let _ = ImageRepository::set_update(&self.repo, visitor.id).await?;

        Ok(())
    }

    async fn check_update(
        &self,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
    ) -> Result<bool, Error> {
        let event = EventRepository::get(&self.repo, event_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", event_id),
            })?;
        let visitor = VisitorRepository::get(&self.repo, event.id, visitor_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", visitor_id),
            })?;
        let is_updated = ImageRepository::check_update(&self.repo, visitor.id).await?;

        Ok(is_updated)
    }
}
