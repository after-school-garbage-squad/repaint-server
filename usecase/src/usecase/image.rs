use std::str::FromStr;

use async_trait::async_trait;
use repaint_server_model::event::Event;
use repaint_server_model::event_image::Image as EventImage;
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_model::visitor_image::{CurrentImage, Image as VisitorImage};
use repaint_server_model::AsyncSafe;
use teloc::inject;

use crate::infra::repo::{EventRepository, ImageRepository, VisitorRepository};
use crate::model::visitor::VisitorIdentification;
use crate::usecase::error::Error;

#[async_trait]
pub trait ImageUsecase: AsyncSafe {
    async fn add_default_image(
        &self,
        subject: String,
        event_id: Id<Event>,
        image_id: Id<EventImage>,
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
        image_id: Id<VisitorImage>,
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
}

#[derive(Debug)]
pub struct ImageUsecaseImpl<R> {
    repo: R,
}

#[inject]
impl<R> ImageUsecaseImpl<R>
where
    R: ImageRepository + EventRepository + VisitorRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R> ImageUsecase for ImageUsecaseImpl<R>
where
    R: ImageRepository + EventRepository + VisitorRepository,
{
    async fn add_default_image(
        &self,
        subject: String,
        event_id: Id<Event>,
        image_id: Id<EventImage>,
    ) -> Result<(), Error> {
        let event = EventRepository::get_event_belong_to_subject(&self.repo, subject, event_id)
            .await?
            .ok_or(Error::UnAuthorized)?;

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
        image_id: Id<VisitorImage>,
    ) -> Result<(), Error> {
        let event = EventRepository::get_event_belong_to_subject(&self.repo, subject, event_id)
            .await?
            .ok_or(Error::UnAuthorized)?;

        let visitor = VisitorRepository::get(&self.repo, event.id, visitor_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} aren't exist", visitor_id),
            })?;

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
        let current_image_id = ImageRepository::get_current_image(&self.repo, visitor.id).await?;

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
        let _ = ImageRepository::set_current_image(&self.repo, visitor.id, image_id).await?;

        Ok(())
    }
}
