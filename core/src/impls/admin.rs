use async_trait::async_trait;
use repaint_server_model::admin::Admin;
use repaint_server_model::id::Id;
use repaint_server_usecase::infra::repo::{AdminRepository, IsUpdated};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

use crate::entity::{admins, events_admins};
use crate::ty::string::ToDatabaseType;
use crate::{Error, SeaOrm};

use super::IsUpdatedExt;

pub fn to_model(m: admins::Model) -> Result<Admin, Error> {
    Ok(Admin {
        id: m.id,
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

    async fn update(&self, admin_id: i32, event_id: i32) -> Result<IsUpdated, Self::Error> {
        events_admins::ActiveModel {
            event_id: Set(event_id),
            admin_id: Set(admin_id),
            ..Default::default()
        }
        .insert(self.con())
        .await
        .to_is_updated()
    }
}
