use std::str::FromStr;

use async_trait::async_trait;
use repaint_server_model::event_image::Image as EventImage;
use repaint_server_model::id::Id;
use repaint_server_model::visitor_image::{CurrentImage, Image as VisitorImage};
use repaint_server_usecase::infra::repo::{ImageRepository, IsUpdated};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, TransactionTrait,
};

use crate::entity::{event_images, events, visitor_images, visitors};
use crate::ty::string::ToDatabaseType;
use crate::{Error, SeaOrm};

use super::IsUpdatedExt;

#[async_trait]
impl ImageRepository for SeaOrm {
    type Error = Error;

    async fn add_default_image(
        &self,
        event_id: i32,
        image_id: Id<EventImage>,
    ) -> Result<IsUpdated, Self::Error> {
        event_images::ActiveModel {
            event_id: Set(event_id),
            image_id: Set(image_id.dty()),
            ..Default::default()
        }
        .insert(self.con())
        .await
        .to_is_updated()
    }

    async fn delete_default_image(
        &self,
        event_id: i32,
        image_id: Id<EventImage>,
    ) -> Result<IsUpdated, Self::Error> {
        let tx = self.con().begin().await?;

        let image = events::Entity::find_by_id(event_id)
            .find_also_related(event_images::Entity)
            .filter(event_images::Column::ImageId.eq(image_id.dty()))
            .one(&tx)
            .await?
            .and_then(|(_, i)| i)
            .unwrap();
        let res = image.delete(&tx).await;
        tx.commit().await?;

        res.to_is_updated()
    }

    async fn upload_visitor_image(
        &self,
        visitor_id: i32,
        image_id: Id<VisitorImage>,
    ) -> Result<IsUpdated, Self::Error> {
        let current_image_id = Id::<CurrentImage>::from_str(&image_id.to_string())
            .ok()
            .unwrap();

        visitor_images::ActiveModel {
            visitor_id: Set(visitor_id),
            image_id: Set(image_id.dty()),
            current_image_id: Set(current_image_id.dty()),
            ..Default::default()
        }
        .insert(self.con())
        .await
        .to_is_updated()
    }

    async fn get_visitor_image(
        &self,
        visitor_id: i32,
    ) -> Result<Option<Id<VisitorImage>>, Self::Error> {
        let Some(image) = visitors::Entity::find_by_id(visitor_id)
            .find_also_related(visitor_images::Entity)
            .one(self.con())
            .await?
            .and_then(|(_, i)| i)
        else {
            return Ok(None);
        };

        Ok(Some(image.image_id.model()))
    }

    async fn list_default_image(&self, event_id: i32) -> Result<Vec<Id<EventImage>>, Self::Error> {
        let ids = events::Entity::find_by_id(event_id)
            .find_with_related(event_images::Entity)
            .all(self.con())
            .await?
            .into_iter()
            .map(|(_, i)| i)
            .flatten()
            .map(|i| i.image_id.model())
            .collect();

        Ok(ids)
    }

    async fn get_current_image(&self, visitor_id: i32) -> Result<Id<CurrentImage>, Self::Error> {
        let image = visitors::Entity::find_by_id(visitor_id)
            .find_also_related(visitor_images::Entity)
            .one(self.con())
            .await?
            .and_then(|(_, i)| i)
            .unwrap();

        Ok(image.current_image_id.model())
    }

    async fn set_current_image(
        &self,
        visitor_id: i32,
        image_id: Id<VisitorImage>,
    ) -> Result<IsUpdated, Self::Error> {
        let current_image_id = Id::<CurrentImage>::from_str(&image_id.to_string())
            .ok()
            .unwrap();

        let tx = self.con().begin().await?;

        let mut image: visitor_images::ActiveModel = visitors::Entity::find_by_id(visitor_id)
            .find_also_related(visitor_images::Entity)
            .one(&tx)
            .await?
            .and_then(|(_, i)| i)
            .unwrap()
            .into();
        image.current_image_id = Set(current_image_id.dty());
        let res = image.update(&tx).await;
        tx.commit().await?;

        res.to_is_updated()
    }
}
