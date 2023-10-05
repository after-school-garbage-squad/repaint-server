use std::str::FromStr;

use async_trait::async_trait;
use repaint_server_model::event_image::Image as EventImage;
use repaint_server_model::id::Id;
use repaint_server_model::visitor_image::{CurrentImage, Image as VisitorImage};
use repaint_server_usecase::infra::repo::{ImageRepository, IsUpdated};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, ModelTrait, QueryFilter, TransactionTrait,
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
            .expect("image not found");
        let res = image.delete(&tx).await;
        tx.commit().await?;

        res.to_is_updated()
    }

    async fn upload_visitor_image(
        &self,
        visitor_id: i32,
        image_id: Id<VisitorImage>,
    ) -> Result<IsUpdated, Self::Error> {
        let current_image_id = Id::<CurrentImage>::from_str(image_id.to_string().as_str())
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

    async fn get_current_image(
        &self,
        visitor_id: i32,
    ) -> Result<Option<Id<CurrentImage>>, Self::Error> {
        let Some(image) = visitors::Entity::find_by_id(visitor_id)
            .find_also_related(visitor_images::Entity)
            .one(self.con())
            .await?
            .and_then(|(_, i)| i)
        else {
            return Ok(None);
        };

        Ok(Some(image.current_image_id.model()))
    }

    async fn set_current_image(
        &self,
        visitor_id: i32,
        image_id: Id<VisitorImage>,
    ) -> Result<IsUpdated, Self::Error> {
        let current_image_id = Id::<CurrentImage>::from_str(image_id.to_string().as_str())
            .ok()
            .unwrap();

        let tx = self.con().begin().await?;

        let mut image: visitor_images::ActiveModel = visitors::Entity::find_by_id(visitor_id)
            .find_also_related(visitor_images::Entity)
            .one(&tx)
            .await?
            .and_then(|(_, i)| i)
            .ok_or(Error::SeaOrm(DbErr::RecordNotFound(
                "visitor_images".into(),
            )))?
            .into();
        image.current_image_id = Set(current_image_id.dty());
        let res = image.update(&tx).await;
        tx.commit().await?;

        res.to_is_updated()
    }

    async fn set_update(&self, visitor_id: i32) -> Result<IsUpdated, Self::Error> {
        let tx = self.con().begin().await?;

        let mut image: visitor_images::ActiveModel = visitors::Entity::find_by_id(visitor_id)
            .find_also_related(visitor_images::Entity)
            .one(&tx)
            .await?
            .and_then(|(_, i)| i)
            .ok_or(Error::SeaOrm(DbErr::RecordNotFound(
                "visitor_images".into(),
            )))?
            .into();
        image.is_updated = Set(true);
        let res = image.update(&tx).await;
        tx.commit().await?;

        res.to_is_updated()
    }

    async fn check_update(&self, visitor_id: i32) -> Result<bool, Self::Error> {
        let image = match visitors::Entity::find_by_id(visitor_id)
            .find_also_related(visitor_images::Entity)
            .one(self.con())
            .await?
            .and_then(|(_, i)| i)
        {
            Some(i) => i,
            None => return Ok(false),
        };

        Ok(image.is_updated)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::*;

    use crate::TestingSeaOrm;

    use super::*;

    #[test_log::test(tokio::test)]
    async fn test_add_default_image() {
        let orm = TestingSeaOrm::new().await;
        let event = orm.make_test_event().await;

        let image_id = Id::<EventImage>::new();

        let _ = ImageRepository::add_default_image(orm.orm(), event.id, image_id.clone())
            .await
            .unwrap();

        let res = ImageRepository::list_default_image(orm.orm(), event.id)
            .await
            .unwrap();

        self::assert_eq!(res, [image_id]);
    }

    #[test_log::test(tokio::test)]
    async fn test_delete_default_image() {
        let orm = TestingSeaOrm::new().await;
        let event = orm.make_test_event().await;

        let image_id = Id::<EventImage>::new();

        let _ = ImageRepository::add_default_image(orm.orm(), event.id, image_id.clone())
            .await
            .unwrap();
        let res1 = ImageRepository::list_default_image(orm.orm(), event.id)
            .await
            .unwrap();
        let _ = ImageRepository::delete_default_image(orm.orm(), event.id, image_id.clone())
            .await
            .unwrap();
        let res2 = ImageRepository::list_default_image(orm.orm(), event.id)
            .await
            .unwrap();

        self::assert_eq!(res1, [image_id]);
        self::assert_eq!(res2, []);
    }

    #[test_log::test(tokio::test)]
    async fn test_visitor_image() {
        let orm = TestingSeaOrm::new().await;
        let event = orm.make_test_event().await;
        let visitor = orm.make_test_visitor(event.id).await;

        let image_id = Id::<VisitorImage>::new();

        let _ = ImageRepository::upload_visitor_image(orm.orm(), visitor.id, image_id.clone())
            .await
            .unwrap();

        let res = ImageRepository::get_visitor_image(orm.orm(), visitor.id)
            .await
            .unwrap();

        self::assert_eq!(res, Some(image_id));
    }

    #[test_log::test(tokio::test)]
    async fn test_get_current_image() {
        let orm = TestingSeaOrm::new().await;
        let event = orm.make_test_event().await;
        let visitor = orm.make_test_visitor(event.id).await;

        let image_id = Id::<VisitorImage>::new();

        let _ = ImageRepository::upload_visitor_image(orm.orm(), visitor.id, image_id.clone())
            .await
            .unwrap();

        let res = ImageRepository::get_current_image(orm.orm(), visitor.id)
            .await
            .unwrap();

        let current_image_id = Id::<CurrentImage>::from_str(image_id.to_string().as_str())
            .ok()
            .unwrap();

        self::assert_eq!(res, Some(current_image_id));
    }

    #[test_log::test(tokio::test)]
    async fn test_set_current_image() {
        let orm = TestingSeaOrm::new().await;
        let event = orm.make_test_event().await;
        let visitor = orm.make_test_visitor(event.id).await;
        let mut images = Vec::new();

        for _ in 0..3 {
            let i = Id::new();
            let _ = ImageRepository::add_default_image(orm.orm(), event.id, i)
                .await
                .unwrap();
            images.push(i);
        }

        let res1 = ImageRepository::get_current_image(orm.orm(), visitor.id)
            .await
            .unwrap();

        let v = Id::new();
        let _ = ImageRepository::upload_visitor_image(orm.orm(), visitor.id, v)
            .await
            .unwrap();

        let res2 = ImageRepository::get_current_image(orm.orm(), visitor.id)
            .await
            .unwrap();
        let current_id2 = Id::<CurrentImage>::from_str(v.to_string().as_str())
            .ok()
            .unwrap();

        let visitor_image_id = Id::<VisitorImage>::from_str(images[1].to_string().as_str())
            .ok()
            .unwrap();
        let _ = ImageRepository::set_current_image(orm.orm(), visitor.id, visitor_image_id.clone())
            .await
            .unwrap();
        let res3 = ImageRepository::get_current_image(orm.orm(), visitor.id)
            .await
            .unwrap();
        let current_id3 = Id::<CurrentImage>::from_str(visitor_image_id.to_string().as_str())
            .ok()
            .unwrap();

        self::assert_eq!(res1, None);
        self::assert_eq!(res2, Some(current_id2));
        self::assert_eq!(res3, Some(current_id3));
    }

    #[test_log::test(tokio::test)]
    async fn test_set_get_update() {
        let orm = TestingSeaOrm::new().await;
        let event = orm.make_test_event().await;
        let visitor = orm.make_test_visitor(event.id).await;
        let v = Id::new();
        let _ = ImageRepository::upload_visitor_image(orm.orm(), visitor.id, v)
            .await
            .unwrap();

        let res1 = ImageRepository::check_update(orm.orm(), visitor.id)
            .await
            .unwrap();

        let _ = ImageRepository::set_update(orm.orm(), visitor.id)
            .await
            .unwrap();

        let res2 = ImageRepository::check_update(orm.orm(), visitor.id)
            .await
            .unwrap();

        self::assert_eq!(res1, false);
        self::assert_eq!(res2, true);
    }
}
