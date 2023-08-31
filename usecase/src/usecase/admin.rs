use async_trait::async_trait;
use email_address::EmailAddress;
use rand::distributions::Alphanumeric;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use repaint_server_model::event::Event;
use repaint_server_model::id::Id;
use repaint_server_model::AsyncSafe;
use teloc::inject;

use crate::infra::email::EmailSender;
use crate::infra::firestore::Firestore;
use crate::infra::repo::{AdminRepository, EventRepository};
use crate::usecase::error::Error;

#[async_trait]
pub trait AdminUsecase: AsyncSafe {
    async fn add_admin(&self, subject: String) -> Result<(), Error>;

    async fn send_email(
        &self,
        subject: String,
        event_id: Id<Event>,
        email: EmailAddress,
    ) -> Result<(), Error>;

    async fn add_operator(&self, subject: String, token: String) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct AdminUsecaseImpl<R, F, E> {
    repo: R,
    firestore: F,
    email: E,
}

#[inject]
impl<R, F, E> AdminUsecaseImpl<R, F, E>
where
    R: AdminRepository + EventRepository,
    F: Firestore,
    E: EmailSender,
{
    pub fn new(repo: R, firestore: F, email: E) -> Self {
        Self {
            repo,
            firestore,
            email,
        }
    }
}

#[async_trait]
impl<R, F, E> AdminUsecase for AdminUsecaseImpl<R, F, E>
where
    R: AdminRepository + EventRepository,
    F: Firestore,
    E: EmailSender,
{
    async fn add_admin(&self, subject: String) -> Result<(), Error> {
        let _ = AdminRepository::add(&self.repo, subject).await?;

        Ok(())
    }

    async fn send_email(
        &self,
        subject: String,
        event_id: Id<Event>,
        email: EmailAddress,
    ) -> Result<(), Error> {
        let event = EventRepository::get_event_belong_to_subject(&self.repo, subject, event_id)
            .await?
            .ok_or(Error::UnAuthorized)?;

        let rng = {
            let rng = rand::thread_rng();
            StdRng::from_rng(rng).unwrap()
        };

        let token = rng
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect::<String>();

        self.email.send(email.clone(), token.clone()).await?;

        self.firestore.set_event_id(token, event.event_id).await?;

        Ok(())
    }

    async fn add_operator(&self, subject: String, token: String) -> Result<(), Error> {
        let admin = AdminRepository::get(&self.repo, subject)
            .await?
            .ok_or(Error::UnAuthorized)?;

        let event_id = self
            .firestore
            .get_event_id(token)
            .await?
            .ok_or(Error::BadRequest {
                message: "This token has already expired or is invalid.".to_string(),
            })?;

        let _ = AdminRepository::update(&self.repo, admin.id, event_id).await?;

        Ok(())
    }
}
