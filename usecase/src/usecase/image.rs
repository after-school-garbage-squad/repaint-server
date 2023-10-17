use std::str::FromStr;

use async_trait::async_trait;
use futures::future::join_all;
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
use crate::infra::repo::{
    EventRepository, ImageRepository, PaletteRepository, TransactionRepository, VisitorRepository,
};
use crate::model::image::{
    CheckUpdateResponse, CheckVisitorImageExistResponse, GetCurrentImageResponse, ListImageItem,
    ListImageResponse, ProxyCurrentImageResponse, ProxyEventImageResponse,
};
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
    ) -> Result<CheckVisitorImageExistResponse, Error>;

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
    ) -> Result<ListImageResponse, Error>;

    async fn get_current_image(
        &self,
        visitor_identification: VisitorIdentification,
    ) -> Result<GetCurrentImageResponse, Error>;

    async fn set_current_image(
        &self,
        visitor_identification: VisitorIdentification,
        image_id: Id<VisitorImage>,
    ) -> Result<(), Error>;

    async fn proxy_current_image(
        &self,
        event_id: Id<Event>,
        image_id: Id<CurrentImage>,
        visitor_id: Id<Visitor>,
    ) -> Result<ProxyCurrentImageResponse, Error>;

    async fn set_update(&self, event_id: Id<Event>, visitor_id: Id<Visitor>) -> Result<(), Error>;

    async fn check_update(
        &self,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
    ) -> Result<CheckUpdateResponse, Error>;

    async fn proxy_event_image(
        &self,
        event_id: Id<Event>,
        image_id: Id<EventImage>,
    ) -> Result<ProxyEventImageResponse, Error>;
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
    R: ImageRepository
        + EventRepository
        + VisitorRepository
        + PaletteRepository
        + TransactionRepository,
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
    R: ImageRepository
        + EventRepository
        + VisitorRepository
        + PaletteRepository
        + TransactionRepository,
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
        let tx = TransactionRepository::begin_transaction(&self.repo).await?;
        let event =
            EventRepository::get_event_belong_to_subject(&self.repo, &tx, subject, event_id)
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
        let _ = ImageRepository::add_default_image(&self.repo, &tx, event.id, image_id).await?;
        let _ = tx.commit().await?;

        Ok(())
    }

    async fn delete_default_image(
        &self,
        subject: String,
        event_id: Id<Event>,
        image_id: Id<EventImage>,
    ) -> Result<(), Error> {
        let tx = TransactionRepository::begin_transaction(&self.repo).await?;
        let event =
            EventRepository::get_event_belong_to_subject(&self.repo, &tx, subject, event_id)
                .await?
                .ok_or(Error::UnAuthorized)?;
        let _ = ImageRepository::delete_default_image(&self.repo, &tx, event.id, image_id).await?;
        let _ = tx.commit().await?;

        Ok(())
    }

    async fn check_visitor_image_exist(
        &self,
        subject: String,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
    ) -> Result<CheckVisitorImageExistResponse, Error> {
        let tx = TransactionRepository::begin_transaction(&self.repo).await?;
        let event =
            EventRepository::get_event_belong_to_subject(&self.repo, &tx, subject, event_id)
                .await?
                .ok_or(Error::UnAuthorized)?;
        let visitor = VisitorRepository::get(&self.repo, &tx, event.id, visitor_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} aren't exist", visitor_id),
            })?;
        let image = ImageRepository::get_visitor_image(&self.repo, &tx, visitor.id).await?;
        let _ = tx.commit().await?;

        Ok(CheckVisitorImageExistResponse { image_id: image })
    }

    async fn upload_visitor_image(
        &self,
        subject: String,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        let tx = TransactionRepository::begin_transaction(&self.repo).await?;
        let event =
            EventRepository::get_event_belong_to_subject(&self.repo, &tx, subject, event_id)
                .await?
                .ok_or(Error::UnAuthorized)?;
        let visitor = VisitorRepository::get(&self.repo, &tx, event.id, visitor_id)
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
        let _ = ImageRepository::upload_visitor_image(&self.repo, &tx, visitor.id, image_id).await?;
        let _ = tx.commit().await?;

        Ok(())
    }

    async fn list_image(
        &self,
        visitor_identification: VisitorIdentification,
    ) -> Result<ListImageResponse, Error> {
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
        let default = ImageRepository::list_default_image(&self.repo, &tx, event.id).await?;
        let vi = ImageRepository::get_visitor_image(&self.repo, &tx, visitor.id).await?;
        let mut image_ids = default
            .iter()
            .filter_map(|&i| Id::<VisitorImage>::from_str(i.to_string().as_str()).ok())
            .collect::<Vec<_>>();
        if let Some(vi) = vi {
            image_ids.push(vi);
        };
        let t = image_ids
            .clone()
            .into_iter()
            .map(|i| self.otp.verify_gray(event.event_id, i));
        let urls = join_all(t)
            .await
            .into_iter()
            .map(|res| res.map(|t| t.full))
            .collect::<Result<Vec<_>, _>>()?;
        let images = image_ids
            .into_iter()
            .zip(urls.into_iter())
            .map(|(i, u)| ListImageItem {
                image_id: i,
                url: u,
            })
            .collect();
        let _ = tx.commit().await?;

        Ok(ListImageResponse { images })
    }

    async fn get_current_image(
        &self,
        visitor_identification: VisitorIdentification,
    ) -> Result<GetCurrentImageResponse, Error> {
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
        let current_image_id = match ImageRepository::get_current_image(&self.repo, &tx, visitor.id)
            .await?
        {
            Some(i) => i,
            None => {
                let default =
                    ImageRepository::list_default_image(&self.repo, &tx, event.id).await?;
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
        let _ = tx.commit().await?;

        Ok(GetCurrentImageResponse {
            image_id: current_image_id,
        })
    }

    async fn set_current_image(
        &self,
        visitor_identification: VisitorIdentification,
        image_id: Id<VisitorImage>,
    ) -> Result<(), Error> {
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
        let _ = ImageRepository::set_current_image(&self.repo, &tx, visitor.id, image_id).await?;
        let _ = tx.commit().await?;

        Ok(())
    }

    async fn proxy_current_image(
        &self,
        event_id: Id<Event>,
        image_id: Id<CurrentImage>,
        visitor_id: Id<Visitor>,
    ) -> Result<ProxyCurrentImageResponse, Error> {
        let tx = TransactionRepository::begin_transaction(&self.repo).await?;
        let event = EventRepository::get(&self.repo, &tx, event_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", event_id),
            })?;
        let visitor = VisitorRepository::get(&self.repo, &tx, event.id, visitor_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", visitor_id),
            })?;
        let token = self
            .otp
            .verify_current(event_id, image_id, visitor_id)
            .await?;
        let _ = VisitorRepository::unset_update(&self.repo, &tx, visitor.id).await?;
        let _ = tx.commit().await?;

        Ok(ProxyCurrentImageResponse { url: token.full })
    }

    async fn set_update(&self, event_id: Id<Event>, visitor_id: Id<Visitor>) -> Result<(), Error> {
        let tx = TransactionRepository::begin_transaction(&self.repo).await?;
        let event = EventRepository::get(&self.repo, &tx, event_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", event_id),
            })?;
        let visitor = VisitorRepository::get(&self.repo, &tx, event.id, visitor_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", visitor_id),
            })?;
        let _ = VisitorRepository::set_update(&self.repo, &tx, visitor.id).await?;
        let _ = tx.commit().await?;

        Ok(())
    }

    async fn check_update(
        &self,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
    ) -> Result<CheckUpdateResponse, Error> {
        let tx = TransactionRepository::begin_transaction(&self.repo).await?;
        let event = EventRepository::get(&self.repo, &tx, event_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", event_id),
            })?;
        let visitor = VisitorRepository::get(&self.repo, &tx, event.id, visitor_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", visitor_id),
            })?;
        let is_updated = VisitorRepository::check_update(&self.repo, &tx, visitor.id).await?;
        let _ = tx.commit().await?;

        Ok(CheckUpdateResponse { is_updated })
    }

    async fn proxy_event_image(
        &self,
        event_id: Id<Event>,
        image_id: Id<EventImage>,
    ) -> Result<ProxyEventImageResponse, Error> {
        let token = self.otp.verify_event(event_id, image_id).await?;

        Ok(ProxyEventImageResponse { url: token.full })
    }
}
