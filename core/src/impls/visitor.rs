use std::str::FromStr;

use async_trait::async_trait;
use chrono::{Duration, NaiveDateTime, Utc};
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_usecase::infra::repo::{IsUpdated, VisitorRepository};
use repaint_server_util::envvar;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, TransactionTrait};

use crate::entity::{event_images, events, visitor_images, visitor_spots, visitors};
use crate::ty::string::ToDatabaseType;
use crate::{Error, SeaOrm};

use super::IsUpdatedExt;

pub fn to_model(m: visitors::Model) -> Result<Visitor, Error> {
    Ok(Visitor {
        id: m.id,
        visitor_id: m.visitor_id.model(),
        registration_id: m.registration_id,
    })
}

#[async_trait]
impl VisitorRepository for SeaOrm {
    type Error = Error;

    async fn create(&self, event_id: i32, registration_id: String) -> Result<Visitor, Self::Error> {
        let tx = self.con().begin().await?;
        let visitor = visitors::ActiveModel {
            event_id: Set(event_id),
            visitor_id: Set(Id::new().dty()),
            registration_id: Set(registration_id),
            ..Default::default()
        }
        .insert(&tx)
        .await?;
        let image = events::Entity::find_by_id(event_id)
            .find_also_related(event_images::Entity)
            .one(&tx)
            .await?
            .and_then(|(_, i)| i)
            .ok_or(Error::SeaOrm(DbErr::RecordNotFound(format!(
                "event_image doesn't found with {}",
                event_id
            ))))?;
        let current_image_id = Id::from_str(image.image_id.model().to_string().as_str()).unwrap();
        let _ = visitor_images::ActiveModel {
            visitor_id: Set(visitor.id),
            current_image_id: Set(current_image_id.dty()),
            ..Default::default()
        }
        .insert(&tx)
        .await?;
        tx.commit().await?;

        to_model(visitor)
    }

    async fn get(
        &self,
        event_id: i32,
        visitor_id: Id<Visitor>,
    ) -> Result<Option<Visitor>, Self::Error> {
        events::Entity::find_by_id(event_id)
            .find_with_related(visitors::Entity)
            .all(self.con())
            .await?
            .into_iter()
            .map(|(_, v)| v)
            .flatten()
            .find(|v| v.visitor_id == visitor_id.dty())
            .map(to_model)
            .transpose()
    }

    async fn delete(&self, visitor_id: i32) -> Result<IsUpdated, Self::Error> {
        visitors::Entity::delete_by_id(visitor_id)
            .exec(self.con())
            .await
            .to_is_updated()
    }

    async fn list(&self, event_id: i32) -> Result<Vec<Visitor>, Self::Error> {
        events::Entity::find_by_id(event_id)
            .find_with_related(visitors::Entity)
            .all(self.con())
            .await?
            .into_iter()
            .map(|(_, v)| v)
            .flatten()
            .map(to_model)
            .collect()
    }

    async fn set_update(&self, visitor_id: i32) -> Result<IsUpdated, Self::Error> {
        let tx = self.con().begin().await?;

        let mut visitor: visitors::ActiveModel = visitors::Entity::find_by_id(visitor_id)
            .one(&tx)
            .await?
            .ok_or(Error::SeaOrm(DbErr::RecordNotFound(format!(
                "visitor doesn't found with {}",
                visitor_id
            ))))?
            .into();
        visitor.is_updated = Set(true);
        let res = visitor.update(&tx).await;
        tx.commit().await?;

        res.to_is_updated()
    }

    async fn unset_update(&self, visitor_id: i32) -> Result<IsUpdated, Self::Error> {
        let tx = self.con().begin().await?;

        let mut visitor: visitors::ActiveModel = visitors::Entity::find_by_id(visitor_id)
            .one(&tx)
            .await?
            .ok_or(Error::SeaOrm(DbErr::RecordNotFound(format!(
                "visitor doesn't found with {}",
                visitor_id
            ))))?
            .into();
        visitor.is_updated = Set(false);
        let res = visitor.update(&tx).await;
        tx.commit().await?;

        res.to_is_updated()
    }

    async fn check_update(&self, visitor_id: i32) -> Result<bool, Self::Error> {
        let visitor = match visitors::Entity::find_by_id(visitor_id)
            .one(self.con())
            .await?
        {
            Some(i) => i,
            None => return Ok(false),
        };

        Ok(visitor.is_updated)
    }

    async fn set_download(&self, visitor_id: i32) -> Result<IsUpdated, Self::Error> {
        let tx = self.con().begin().await?;

        let mut visitor: visitors::ActiveModel = visitors::Entity::find_by_id(visitor_id)
            .one(&tx)
            .await?
            .ok_or(Error::SeaOrm(DbErr::RecordNotFound(format!(
                "visitor doesn't found with {}",
                visitor_id
            ))))?
            .into();
        visitor.is_downloadable = Set(true);
        let res = visitor.update(&tx).await;
        tx.commit().await?;

        res.to_is_updated()
    }

    async fn check_download(&self, visitor_id: i32) -> Result<bool, Self::Error> {
        let visitor = match visitors::Entity::find_by_id(visitor_id)
            .one(self.con())
            .await?
        {
            Some(i) => i,
            None => return Ok(false),
        };

        Ok(visitor.is_downloadable)
    }

    async fn get_last_droped_at(
        &self,
        visitor_id: i32,
    ) -> Result<Option<NaiveDateTime>, Self::Error> {
        let visitor = match visitors::Entity::find_by_id(visitor_id)
            .one(self.con())
            .await?
        {
            Some(i) => i,
            None => return Ok(None),
        };

        Ok(visitor.last_droped_at)
    }

    async fn get_last_picked_at(
        &self,
        visitor_id: i32,
        spot_id: i32,
    ) -> Result<Option<NaiveDateTime>, Self::Error> {
        let visitor_spot = match visitors::Entity::find_by_id(visitor_id)
            .find_also_related(visitor_spots::Entity)
            .filter(visitor_spots::Column::SpotId.eq(spot_id))
            .one(self.con())
            .await?
            .and_then(|(_, s)| s)
        {
            Some(s) => s,
            None => return Ok(None),
        };

        Ok(visitor_spot.last_picked_at)
    }

    async fn get_visitors(&self, spot_id: i32) -> Result<Vec<i32>, Self::Error> {
        let now = Utc::now().naive_utc();
        let visitors = visitor_spots::Entity::find()
            .filter(visitor_spots::Column::SpotId.eq(spot_id))
            .all(self.con())
            .await?;

        Ok(visitors
            .into_iter()
            .map(|v| {
                if now - v.last_scanned_at <= Duration::seconds(envvar("VISITOR_SPOT_TIMEOUT", 300))
                {
                    Some(v.id)
                } else {
                    None
                }
            })
            .flatten()
            .collect())
    }
}

#[cfg(test)]
pub(crate) mod test {
    use pretty_assertions::*;
    use repaint_server_model::event_image::Image as EventImage;

    use crate::TestingSeaOrm;

    use super::*;

    impl TestingSeaOrm {
        pub(crate) async fn make_test_visitor(
            &self,
            event_id: i32,
            image_id: Id<EventImage>,
        ) -> Visitor {
            let tx = self.orm().con().begin().await.unwrap();
            let visitor = visitors::ActiveModel {
                event_id: Set(event_id),
                visitor_id: Set(Id::new().dty()),
                registration_id: Set("eXaMpLeReGiStRaTiOnId0123456789".into()),
                ..Default::default()
            }
            .insert(&tx)
            .await
            .unwrap();
            let current_image_id = Id::from_str(image_id.to_string().as_str()).unwrap();
            let _ = visitor_images::ActiveModel {
                visitor_id: Set(visitor.id),
                current_image_id: Set(current_image_id.dty()),
                ..Default::default()
            }
            .insert(&tx)
            .await
            .unwrap();
            tx.commit().await.unwrap();

            to_model(visitor).unwrap()
        }
    }

    #[test_log::test(tokio::test)]
    async fn test_get() {
        let orm = TestingSeaOrm::new().await;
        let event = orm.make_test_event().await;
        let image_id = orm.make_test_default_image(event.id).await;
        let visitor = orm.make_test_visitor(event.id, image_id).await;

        let res = VisitorRepository::get(orm.orm(), event.id, visitor.visitor_id)
            .await
            .unwrap();

        self::assert_eq!(res, Some(visitor));
    }

    #[test_log::test(tokio::test)]
    async fn test_list() {
        async fn test(q: u8) {
            let orm = TestingSeaOrm::new().await;
            let event = orm.make_test_event().await;
            let image_id = orm.make_test_default_image(event.id).await;
            let mut visitors = Vec::<Visitor>::new();
            for _ in 0..q {
                visitors.push(orm.make_test_visitor(event.id, image_id).await);
            }

            let res = VisitorRepository::list(orm.orm(), event.id).await.unwrap();

            self::assert_eq!(res, visitors);
        }

        test(1).await;
        test(2).await;
        test(3).await;
    }

    #[test_log::test(tokio::test)]
    async fn test_delete() {
        let orm = TestingSeaOrm::new().await;
        let event = orm.make_test_event().await;
        let image_id = orm.make_test_default_image(event.id).await;
        let mut visitors = Vec::new();
        for _ in 0..3 {
            visitors.push(orm.make_test_visitor(event.id, image_id).await);
        }
        let _ = VisitorRepository::delete(orm.orm(), visitors[1].id)
            .await
            .unwrap();
        visitors.remove(1);

        let res = VisitorRepository::list(orm.orm(), event.id).await.unwrap();

        self::assert_eq!(res, visitors);
    }

    #[test_log::test(tokio::test)]
    async fn test_set_get_unset_update() {
        let orm = TestingSeaOrm::new().await;
        let event = orm.make_test_event().await;
        let image_id = orm.make_test_default_image(event.id).await;
        let visitor = orm.make_test_visitor(event.id, image_id).await;

        let res1 = VisitorRepository::check_update(orm.orm(), visitor.id)
            .await
            .unwrap();

        let _ = VisitorRepository::set_update(orm.orm(), visitor.id)
            .await
            .unwrap();

        let res2 = VisitorRepository::check_update(orm.orm(), visitor.id)
            .await
            .unwrap();

        let _ = VisitorRepository::unset_update(orm.orm(), visitor.id)
            .await
            .unwrap();

        let res3 = VisitorRepository::check_update(orm.orm(), visitor.id)
            .await
            .unwrap();

        self::assert_eq!(res1, false);
        self::assert_eq!(res2, true);
        self::assert_eq!(res3, false);
    }

    #[test_log::test(tokio::test)]
    async fn test_set_get_download() {
        let orm = TestingSeaOrm::new().await;
        let event = orm.make_test_event().await;
        let image_id = orm.make_test_default_image(event.id).await;
        let visitor = orm.make_test_visitor(event.id, image_id).await;

        let res1 = VisitorRepository::check_download(orm.orm(), visitor.id)
            .await
            .unwrap();

        let _ = VisitorRepository::set_download(orm.orm(), visitor.id)
            .await
            .unwrap();

        let res2 = VisitorRepository::check_download(orm.orm(), visitor.id)
            .await
            .unwrap();

        self::assert_eq!(res1, false);
        self::assert_eq!(res2, true);
    }
}
