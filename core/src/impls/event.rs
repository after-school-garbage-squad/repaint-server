use async_trait::async_trait;
use repaint_server_model::event::{Contact, Event};
use repaint_server_model::id::Id;
use repaint_server_usecase::infra::repo::{EventRepository, IsUpdated};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, TransactionTrait};

use crate::entity::{admins, events};
use crate::ty::json::AsJson;
use crate::ty::string::ToDatabaseType;
use crate::{Error, SeaOrm};

use super::IsUpdatedExt;

pub fn to_model(m: events::Model) -> Result<Event, Error> {
    Ok(Event {
        id: m.id,
        event_id: m.event_id.model(),
        name: m.name,
        hp_url: m.hp_url,
        contact: Contact {
            name: m.contact.clone().model().name,
            email: m.contact.clone().model().email,
            phone: m.contact.model().phone,
        },
    })
}

#[async_trait]
impl EventRepository for SeaOrm {
    type Error = Error;

    async fn get_event_belong_to_subject(
        &self,
        subject: String,
        event_id: Id<Event>,
    ) -> Result<Option<Event>, Self::Error> {
        admins::Entity::find()
            .filter(admins::Column::Subject.eq(subject))
            .find_with_related(events::Entity)
            .all(self.con())
            .await?
            .into_iter()
            .map(|(_, events)| events)
            .flatten()
            .find(|e| e.event_id == event_id.dty())
            .map(to_model)
            .transpose()
    }

    async fn create(
        &self,
        name: String,
        hp_url: String,
        contact: Contact,
    ) -> Result<Event, Self::Error> {
        let event = events::ActiveModel {
            event_id: Set(Id::new().dty()),
            name: Set(name),
            hp_url: Set(hp_url),
            contact: Set(AsJson(contact)),
            ..Default::default()
        }
        .insert(self.con())
        .await?;

        to_model(event)
    }

    async fn delete(&self, event_id: i32) -> Result<IsUpdated, Self::Error> {
        events::Entity::delete_by_id(event_id)
            .exec(self.con())
            .await
            .to_is_updated()
    }

    async fn list(&self, subject: String) -> Result<Vec<Event>, Self::Error> {
        admins::Entity::find()
            .filter(admins::Column::Subject.eq(subject.dty()))
            .find_with_related(events::Entity)
            .all(self.con())
            .await?
            .into_iter()
            .map(|(_, events)| events)
            .flatten()
            .map(to_model)
            .collect()
    }

    async fn update(
        &self,
        event_id: i32,
        name: String,
        hp_url: String,
        contact: Contact,
    ) -> Result<Event, Self::Error> {
        let tx = self.con().begin().await?;

        let mut event: events::ActiveModel = events::Entity::find_by_id(event_id)
            .one(&tx)
            .await?
            .unwrap()
            .into();
        event.name = Set(name);
        event.hp_url = Set(hp_url);
        event.contact = Set(AsJson(contact));
        let event = event.update(&tx).await?;

        tx.commit().await?;

        to_model(event)
    }

    async fn get(&self, event_id: Id<Event>) -> Result<Option<Event>, Self::Error> {
        events::Entity::find()
            .filter(events::Column::EventId.eq(event_id.dty()))
            .one(self.con())
            .await?
            .map(to_model)
            .transpose()
    }
}
