use async_trait::async_trait;
use repaint_server_model::admin::Admin;
use repaint_server_model::event::Event;
use repaint_server_model::id::Id;
use repaint_server_usecase::infra::repo::{AdminRepository, IsUpdated};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, TransactionTrait};

use crate::entity::{admins, events, events_admins};
use crate::ty::string::ToDatabaseType;
use crate::{Error, SeaOrm};

use super::IsUpdatedExt;

pub fn to_model(m: admins::Model) -> Result<Admin, Error> {
    Ok(Admin {
        admin_id: m.admin_id.model(),
        subject: m.subject,
    })
}

#[async_trait]
impl AdminRepository for SeaOrm {
    type Error = Error;

    async fn add(&self, subject: String) -> Result<IsUpdated, Self::Error> {
        admins::ActiveModel {
            subject: Set(subject),
            admin_id: Set(Id::new().dty()),
            ..Default::default()
        }
        .insert(self.con())
        .await
        .to_is_updated()
    }

    async fn get(&self, subject: String) -> Result<Option<Admin>, Self::Error> {
        admins::Entity::find()
            .filter(admins::Column::Subject.eq(subject))
            .one(self.con())
            .await?
            .map(to_model)
            .transpose()
    }

    async fn update(
        &self,
        admin_id: Id<Admin>,
        event_id: Id<Event>,
    ) -> Result<IsUpdated, Self::Error> {
        let tx = self.con().begin().await?;

        let admin = admins::Entity::find()
            .filter(admins::Column::AdminId.eq(admin_id.dty()))
            .one(&tx)
            .await?
            .unwrap();
        let event = events::Entity::find()
            .filter(events::Column::EventId.eq(event_id.dty()))
            .one(&tx)
            .await?
            .unwrap();
        let admin = events_admins::ActiveModel {
            event_id: Set(event.id),
            admin_id: Set(admin.id),
            ..Default::default()
        }
        .insert(&tx)
        .await;
        tx.commit().await?;

        admin.to_is_updated()
    }
}
