use std::str::FromStr;

use async_trait::async_trait;
use repaint_server_model::event_image::Image as EventImage;
use repaint_server_model::id::Id;
use repaint_server_model::visitor_image::{CurrentImage, Image as VisitorImage};
use repaint_server_usecase::infra::repo::{ImageRepository, IsUpdated};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, ModelTrait,
    QueryFilter, TransactionTrait,
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
        tx: &DatabaseTransaction,
        event_id: i32,
        image_id: Id<EventImage>,
    ) -> Result<IsUpdated, Self::Error> {
        event_images::ActiveModel {
            event_id: Set(event_id),
            image_id: Set(image_id.dty()),
            ..Default::default()
        }
        .insert(tx)
        .await
        .to_is_updated()
    }

    async fn delete_default_image(
        &self,
        txn: &DatabaseTransaction,
        event_id: i32,
        image_id: Id<EventImage>,
    ) -> Result<IsUpdated, Self::Error> {
        let tx = txn.begin().await?;
        let image = events::Entity::find_by_id(event_id)
            .find_also_related(event_images::Entity)
            .filter(event_images::Column::ImageId.eq(image_id.dty()))
            .one(&tx)
            .await?
            .and_then(|(_, i)| i)
            .ok_or(Error::SeaOrm(DbErr::RecordNotFound(format!(
                "event_image doesn't found with {}",
                image_id
            ))))?;
        let res = image.delete(&tx).await;
        tx.commit().await?;

        res.to_is_updated()
    }

    async fn upload_visitor_image(
        &self,
        txn: &DatabaseTransaction,
        visitor_id: i32,
        image_id: Id<VisitorImage>,
    ) -> Result<IsUpdated, Self::Error> {
        let tx = txn.begin().await?;
        let current_image_id = Id::<CurrentImage>::from_str(image_id.to_string().as_str())
            .ok()
            .unwrap();
        let mut visitor_image: visitor_images::ActiveModel =
            visitors::Entity::find_by_id(visitor_id)
                .find_also_related(visitor_images::Entity)
                .one(&tx)
                .await?
                .and_then(|(_, i)| i)
                .ok_or(Error::SeaOrm(DbErr::RecordNotFound(format!(
                    "visitor_image doesn't found with {}",
                    visitor_id
                ))))?
                .into();
        visitor_image.image_id = Set(Some(image_id.dty()));
        visitor_image.current_image_id = Set(current_image_id.dty());
        let res = visitor_image.update(&tx).await;
        tx.commit().await?;

        res.to_is_updated()
    }

    async fn get_visitor_image(
        &self,
        tx: &DatabaseTransaction,
        visitor_id: i32,
    ) -> Result<Option<Id<VisitorImage>>, Self::Error> {
        let Some(image) = visitors::Entity::find_by_id(visitor_id)
            .find_also_related(visitor_images::Entity)
            .one(tx)
            .await?
            .and_then(|(_, i)| i)
        else {
            return Ok(None);
        };

        Ok(image.image_id.map(|i| i.model()))
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

    async fn list_default_image_with_tx(
        &self,
        tx: &DatabaseTransaction,
        event_id: i32,
    ) -> Result<Vec<Id<EventImage>>, Self::Error> {
        let ids = events::Entity::find_by_id(event_id)
            .find_with_related(event_images::Entity)
            .all(tx)
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
        tx: &DatabaseTransaction,
        visitor_id: i32,
    ) -> Result<Option<Id<CurrentImage>>, Self::Error> {
        let Some(image) = visitors::Entity::find_by_id(visitor_id)
            .find_also_related(visitor_images::Entity)
            .one(tx)
            .await?
            .and_then(|(_, i)| i)
        else {
            return Ok(None);
        };

        Ok(Some(image.current_image_id.model()))
    }

    async fn set_current_image(
        &self,
        txn: &DatabaseTransaction,
        visitor_id: i32,
        image_id: Id<VisitorImage>,
    ) -> Result<IsUpdated, Self::Error> {
        let current_image_id = Id::<CurrentImage>::from_str(image_id.to_string().as_str())
            .ok()
            .unwrap();
        let tx = txn.begin().await?;
        let mut image: visitor_images::ActiveModel = visitors::Entity::find_by_id(visitor_id)
            .find_also_related(visitor_images::Entity)
            .one(&tx)
            .await?
            .and_then(|(_, i)| i)
            .ok_or(Error::SeaOrm(DbErr::RecordNotFound(format!(
                "visitor_image doesn't found with {}",
                visitor_id
            ))))?
            .into();
        image.current_image_id = Set(current_image_id.dty());
        let res = image.update(&tx).await;
        tx.commit().await?;

        res.to_is_updated()
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::*;
    use repaint_server_usecase::infra::repo::TransactionRepository;

    use crate::TestingSeaOrm;

    use super::*;

    impl TestingSeaOrm {
        pub(crate) async fn make_test_default_image(
            &self,
            tx: &DatabaseTransaction,
            event_id: i32,
        ) -> Id<EventImage> {
            let image_id = Id::new();
            let _ = ImageRepository::add_default_image(self.orm(), &tx, event_id, image_id.clone())
                .await
                .unwrap();

            image_id
        }
    }

    #[test_log::test(tokio::test)]
    async fn test_add_default_image() {
        let orm = TestingSeaOrm::new().await;
        let event = orm.make_test_event().await;
        let tx = TransactionRepository::begin_transaction(orm.orm())
            .await
            .unwrap();
        let image_id = Id::<EventImage>::new();
        let _ = ImageRepository::add_default_image(orm.orm(), &tx, event.id, image_id.clone())
            .await
            .unwrap();
        let res = ImageRepository::list_default_image_with_tx(orm.orm(), &tx, event.id)
            .await
            .unwrap();
        let _ = tx.commit().await.unwrap();

        self::assert_eq!(res, [image_id]);
    }

    #[test_log::test(tokio::test)]
    async fn test_delete_default_image() {
        let orm = TestingSeaOrm::new().await;
        let event = orm.make_test_event().await;
        let tx = TransactionRepository::begin_transaction(orm.orm())
            .await
            .unwrap();

        let image_id = Id::<EventImage>::new();

        let _ = ImageRepository::add_default_image(orm.orm(), &tx, event.id, image_id.clone())
            .await
            .unwrap();
        let res1 = ImageRepository::list_default_image_with_tx(orm.orm(), &tx, event.id)
            .await
            .unwrap();
        let _ = ImageRepository::delete_default_image(orm.orm(), &tx, event.id, image_id.clone())
            .await
            .unwrap();
        let res2 = ImageRepository::list_default_image_with_tx(orm.orm(), &tx, event.id)
            .await
            .unwrap();
        let _ = tx.commit().await.unwrap();

        self::assert_eq!(res1, [image_id]);
        self::assert_eq!(res2, []);
    }

    #[test_log::test(tokio::test)]
    async fn test_visitor_image() {
        let orm = TestingSeaOrm::new().await;
        let tx = TransactionRepository::begin_transaction(orm.orm())
            .await
            .unwrap();
        let event = orm.make_test_event().await;
        let image_id = orm.make_test_default_image(&tx, event.id).await;
        let visitor = orm.make_test_visitor(event.id, image_id).await;
        let image_id = Id::<VisitorImage>::new();
        let _ = ImageRepository::upload_visitor_image(orm.orm(), &tx, visitor.id, image_id.clone())
            .await
            .unwrap();
        let res = ImageRepository::get_visitor_image(orm.orm(), &tx, visitor.id)
            .await
            .unwrap();
        let _ = tx.commit().await.unwrap();

        self::assert_eq!(res, Some(image_id));
    }

    #[test_log::test(tokio::test)]
    async fn test_get_current_image() {
        let orm = TestingSeaOrm::new().await;
        let tx = TransactionRepository::begin_transaction(orm.orm())
            .await
            .unwrap();
        let event = orm.make_test_event().await;
        let image_id = orm.make_test_default_image(&tx, event.id).await;
        let visitor = orm.make_test_visitor(event.id, image_id).await;
        let i = Id::<VisitorImage>::new();
        let res1 = ImageRepository::get_current_image(orm.orm(), &tx, visitor.id)
            .await
            .unwrap();
        let _ = ImageRepository::upload_visitor_image(orm.orm(), &tx, visitor.id, i.clone())
            .await
            .unwrap();
        let res2 = ImageRepository::get_current_image(orm.orm(), &tx, visitor.id)
            .await
            .unwrap();
        let current_image_id = Id::<CurrentImage>::from_str(image_id.to_string().as_str())
            .ok()
            .unwrap();
        let c = Id::<CurrentImage>::from_str(i.to_string().as_str())
            .ok()
            .unwrap();
        let _ = tx.commit().await.unwrap();

        self::assert_eq!(res1, Some(current_image_id));
        self::assert_eq!(res2, Some(c));
    }

    #[test_log::test(tokio::test)]
    async fn test_set_current_image() {
        let orm = TestingSeaOrm::new().await;
        let tx = TransactionRepository::begin_transaction(orm.orm())
            .await
            .unwrap();
        let event = orm.make_test_event().await;
        let image_id = orm.make_test_default_image(&tx, event.id).await;
        let visitor = orm.make_test_visitor(event.id, image_id).await;
        let mut images = Vec::new();
        for _ in 0..3 {
            let i = Id::new();
            let _ = ImageRepository::add_default_image(orm.orm(), &tx, event.id, i)
                .await
                .unwrap();
            images.push(i);
        }
        let res1 = ImageRepository::get_current_image(orm.orm(), &tx, visitor.id)
            .await
            .unwrap();
        let current_id1 = Id::<CurrentImage>::from_str(image_id.to_string().as_str())
            .ok()
            .unwrap();
        let v = Id::new();
        let _ = ImageRepository::upload_visitor_image(orm.orm(), &tx, visitor.id, v)
            .await
            .unwrap();
        let res2 = ImageRepository::get_current_image(orm.orm(), &tx, visitor.id)
            .await
            .unwrap();
        let current_id2 = Id::<CurrentImage>::from_str(v.to_string().as_str())
            .ok()
            .unwrap();
        let visitor_image_id = Id::<VisitorImage>::from_str(images[1].to_string().as_str())
            .ok()
            .unwrap();
        let _ = ImageRepository::set_current_image(
            orm.orm(),
            &tx,
            visitor.id,
            visitor_image_id.clone(),
        )
        .await
        .unwrap();
        let res3 = ImageRepository::get_current_image(orm.orm(), &tx, visitor.id)
            .await
            .unwrap();
        let current_id3 = Id::<CurrentImage>::from_str(visitor_image_id.to_string().as_str())
            .ok()
            .unwrap();
        let _ = tx.commit().await.unwrap();

        self::assert_eq!(res1, Some(current_id1));
        self::assert_eq!(res2, Some(current_id2));
        self::assert_eq!(res3, Some(current_id3));
    }
}
