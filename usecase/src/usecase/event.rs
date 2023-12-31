use async_trait::async_trait;
use futures::future::join_all;
use repaint_server_model::event::{Contact, Event};
use repaint_server_model::id::Id;
use repaint_server_model::AsyncSafe;
use teloc::inject;

use crate::infra::firestore::Firestore;
use crate::infra::repo::{
    AdminRepository, EventRepository, ImageRepository, SpotRepository, TransactionRepository,
    VisitorRepository,
};
use crate::model::event::{CreateEventResponse, EventResponse, UpdateEventResponse};
use crate::usecase::error::Error;

#[async_trait]
pub trait EventUsecase: AsyncSafe {
    async fn create_event(
        &self,
        subject: String,
        name: String,
        hp_url: String,
        contact: Contact,
    ) -> Result<CreateEventResponse, Error>;

    async fn delete_event(&self, subject: String, event_id: Id<Event>) -> Result<(), Error>;

    async fn list_event(&self, subject: String) -> Result<Vec<EventResponse>, Error>;

    async fn update_event(
        &self,
        subject: String,
        event_id: Id<Event>,
        name: String,
        hp_url: String,
        contact: Contact,
    ) -> Result<UpdateEventResponse, Error>;
}

#[derive(Debug)]
pub struct EventUsecaseImpl<R, F> {
    repo: R,
    firestore: F,
}

#[inject]
impl<R, F> EventUsecaseImpl<R, F>
where
    R: EventRepository
        + AdminRepository
        + SpotRepository
        + ImageRepository
        + VisitorRepository
        + TransactionRepository,
    F: Firestore,
{
    pub fn new(repo: R, firestore: F) -> Self {
        Self { repo, firestore }
    }
}

#[async_trait]
impl<R, F> EventUsecase for EventUsecaseImpl<R, F>
where
    R: EventRepository
        + AdminRepository
        + SpotRepository
        + ImageRepository
        + VisitorRepository
        + TransactionRepository,
    F: Firestore,
{
    async fn create_event(
        &self,
        subject: String,
        name: String,
        hp_url: String,
        contact: Contact,
    ) -> Result<CreateEventResponse, Error> {
        let tx = TransactionRepository::begin_transaction(&self.repo).await?;
        let admin = AdminRepository::get_with_tx(&self.repo, &tx, subject)
            .await?
            .ok_or(Error::UnAuthorized)?;
        if name.chars().count() > 32 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 32 chars", name),
            });
        }
        if hp_url.chars().count() > 2083 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 2083 chars", hp_url),
            });
        }
        if contact.name.chars().count() > 32 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 32 chars", contact.name),
            });
        }
        if contact.email.as_str().chars().count() > 80 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 80 chars", contact.email),
            });
        }
        if contact.phone.chars().count() > 11 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 11 chars", contact.phone),
            });
        }
        let event = EventRepository::create(&self.repo, &tx, name, hp_url, contact).await?;
        let _ = AdminRepository::update(&self.repo, &tx, admin.id, event.id).await?;
        let _ = tx.commit().await?;

        Ok(CreateEventResponse {
            event_id: event.event_id,
            name: event.name,
            hp_url: event.hp_url,
            contact: event.contact,
        })
    }

    async fn delete_event(&self, subject: String, event_id: Id<Event>) -> Result<(), Error> {
        let tx = TransactionRepository::begin_transaction(&self.repo).await?;
        let event =
            EventRepository::get_event_belong_to_subject(&self.repo, &tx, subject, event_id)
                .await?
                .ok_or(Error::UnAuthorized)?;
        let _ = EventRepository::delete(&self.repo, &tx, event.id).await?;
        let _ = self.firestore.delete(event.event_id).await?;
        let _ = tx.commit().await?;

        Ok(())
    }

    async fn list_event(&self, subject: String) -> Result<Vec<EventResponse>, Error> {
        let tx = TransactionRepository::begin_transaction(&self.repo).await?;
        let events = EventRepository::list(&self.repo, &tx, subject).await?;
        let s = events
            .iter()
            .map(|e| SpotRepository::list(&self.repo, e.id));
        let spots = join_all(s)
            .await
            .into_iter()
            .collect::<Result<Vec<Vec<_>>, _>>()?;
        let i = events
            .iter()
            .map(|e| ImageRepository::list_default_image(&self.repo, e.id));
        let images = join_all(i)
            .await
            .into_iter()
            .collect::<Result<Vec<Vec<_>>, _>>()?;
        let _ = tx.commit().await?;

        Ok(events
            .into_iter()
            .zip(spots)
            .zip(images)
            .map(|((e, s), i)| EventResponse {
                event_id: e.event_id,
                name: e.name,
                hp_url: e.hp_url,
                contact: e.contact,
                spots: s,
                images: i,
            })
            .collect())
    }

    async fn update_event(
        &self,
        subject: String,
        event_id: Id<Event>,
        name: String,
        hp_url: String,
        contact: Contact,
    ) -> Result<UpdateEventResponse, Error> {
        let tx = TransactionRepository::begin_transaction(&self.repo).await?;
        let event =
            EventRepository::get_event_belong_to_subject(&self.repo, &tx, subject, event_id)
                .await?
                .ok_or(Error::UnAuthorized)?;
        if name.chars().count() > 32 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 32 chars", name),
            });
        }
        if hp_url.chars().count() > 2083 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 2083 chars", hp_url),
            });
        }
        if contact.name.chars().count() > 32 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 32 chars", contact.name),
            });
        }
        if contact.email.as_str().chars().count() > 80 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 80 chars", contact.email),
            });
        }
        if contact.phone.chars().count() > 11 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 11 chars", contact.phone),
            });
        }
        let Some(event) =
            EventRepository::update(&self.repo, &tx, event.id, name, hp_url, contact).await?
        else {
            return Err(Error::BadRequest {
                message: format!("{} is not found", event_id),
            });
        };
        let _ = tx.commit().await?;

        Ok(UpdateEventResponse {
            event_id: event.event_id,
            name: event.name,
            hp_url: event.hp_url,
            contact: event.contact,
        })
    }
}
