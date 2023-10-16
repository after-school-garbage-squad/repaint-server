use async_trait::async_trait;
use chrono::NaiveDateTime;
use repaint_server_model::event_spot::EventSpot;
use repaint_server_model::id::Id;
use repaint_server_usecase::infra::repo::{IsUpdated, SpotRepository};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, ModelTrait,
    QueryFilter, TransactionTrait,
};

use crate::entity::{event_spots, events, visitor_spots, visitors};
use crate::ty::string::ToDatabaseType;
use crate::{Error, SeaOrm};

use super::IsUpdatedExt;

pub fn to_model(m: event_spots::Model) -> Result<EventSpot, Error> {
    Ok(EventSpot {
        id: m.id,
        spot_id: m.spot_id.model(),
        name: m.name,
        is_pick: m.is_pick,
        bonus: m.bonus,
        hw_id: m.hw_id,
        service_uuid: m.service_uuid,
    })
}

#[async_trait]
impl SpotRepository for SeaOrm {
    type Error = Error;

    async fn register(
        &self,
        event_id: i32,
        name: String,
        hw_id: String,
        service_uuid: String,
    ) -> Result<EventSpot, Self::Error> {
        let spot = event_spots::ActiveModel {
            event_id: Set(event_id),
            spot_id: Set(Id::new().dty()),
            name: Set(name),
            hw_id: Set(hw_id),
            service_uuid: Set(service_uuid),
            ..Default::default()
        }
        .insert(self.con())
        .await?;

        to_model(spot)
    }

    async fn list(
        &self,
        tx: &DatabaseTransaction,
        event_id: i32,
    ) -> Result<Vec<EventSpot>, Self::Error> {
        events::Entity::find_by_id(event_id)
            .find_with_related(event_spots::Entity)
            .all(tx)
            .await?
            .into_iter()
            .map(|(_, s)| s)
            .flatten()
            .map(to_model)
            .collect()
    }

    async fn get_by_beacon(
        &self,
        event_id: i32,
        hw_id: String,
    ) -> Result<Option<EventSpot>, Self::Error> {
        let Some(spot) = events::Entity::find_by_id(event_id)
            .find_with_related(event_spots::Entity)
            .all(self.con())
            .await?
            .into_iter()
            .map(|(_, s)| s)
            .flatten()
            .find(|s| s.hw_id == hw_id)
        else {
            return Ok(None);
        };

        to_model(spot).map(Some)
    }

    async fn get_by_qr(
        &self,
        event_id: i32,
        spot_id: Id<EventSpot>,
    ) -> Result<Option<EventSpot>, Self::Error> {
        events::Entity::find_by_id(event_id)
            .find_with_related(event_spots::Entity)
            .all(self.con())
            .await?
            .into_iter()
            .map(|(_, s)| s)
            .flatten()
            .find(|s| s.spot_id == spot_id.dty())
            .map(to_model)
            .transpose()
    }

    async fn get_by_id(&self, spot_id: i32) -> Result<Option<EventSpot>, Self::Error> {
        event_spots::Entity::find_by_id(spot_id)
            .one(self.con())
            .await?
            .map(to_model)
            .transpose()
    }

    async fn update(
        &self,
        event_id: i32,
        spot_id: Id<EventSpot>,
        name: String,
        is_pick: bool,
    ) -> Result<Option<EventSpot>, Self::Error> {
        let mut spot: event_spots::ActiveModel = match events::Entity::find_by_id(event_id)
            .find_with_related(event_spots::Entity)
            .all(self.con())
            .await?
            .into_iter()
            .map(|(_, s)| s)
            .flatten()
            .find(|s| s.spot_id == spot_id.dty())
        {
            Some(m) => m.into(),
            None => return Ok(None),
        };
        spot.name = Set(name);
        spot.is_pick = Set(is_pick);
        let spot = spot.update(self.con()).await?;

        to_model(spot).map(Some)
    }

    async fn delete(
        &self,
        event_id: i32,
        spot_id: Id<EventSpot>,
    ) -> Result<IsUpdated, Self::Error> {
        let tx = self.con().begin().await?;

        let spot = events::Entity::find_by_id(event_id)
            .find_with_related(event_spots::Entity)
            .all(&tx)
            .await?
            .into_iter()
            .map(|(_, s)| s)
            .flatten()
            .find(|s| s.spot_id == spot_id.dty())
            .ok_or(Error::SeaOrm(DbErr::RecordNotFound(format!(
                "event_spot doesn't found with {}",
                spot_id
            ))))?;
        let res = spot.delete(&tx).await;
        tx.commit().await?;

        res.to_is_updated()
    }

    async fn get_bonus_state(
        &self,
        event_id: i32,
        spot_id: Id<EventSpot>,
    ) -> Result<bool, Self::Error> {
        events::Entity::find_by_id(event_id)
            .find_with_related(event_spots::Entity)
            .all(self.con())
            .await?
            .into_iter()
            .map(|(_, s)| s)
            .flatten()
            .find(|s| s.spot_id == spot_id.dty())
            .ok_or(Error::SeaOrm(DbErr::RecordNotFound(format!(
                "event_spot doesn't found with {}",
                spot_id
            ))))
            .map(to_model)?
            .and_then(|s| Ok(s.bonus))
    }

    async fn set_bonus_state(
        &self,
        event_id: i32,
        spot_id: Id<EventSpot>,
        is_bonus: bool,
    ) -> Result<IsUpdated, Self::Error> {
        let tx = self.con().begin().await?;

        let mut spot: event_spots::ActiveModel = events::Entity::find_by_id(event_id)
            .find_with_related(event_spots::Entity)
            .all(&tx)
            .await?
            .into_iter()
            .map(|(_, s)| s)
            .flatten()
            .find(|s| s.spot_id == spot_id.dty())
            .ok_or(Error::SeaOrm(DbErr::RecordNotFound(format!(
                "event_spot doesn't found with {}",
                spot_id
            ))))?
            .into();
        spot.bonus = Set(is_bonus);
        let res = spot.update(&tx).await;
        tx.commit().await?;

        res.to_is_updated()
    }

    async fn scanned(
        &self,
        visitor_id: i32,
        spot_id: i32,
        now: NaiveDateTime,
    ) -> Result<IsUpdated, Self::Error> {
        let tx = self.con().begin().await?;
        let visitor_spot = visitors::Entity::find_by_id(visitor_id)
            .find_also_related(visitor_spots::Entity)
            .filter(visitor_spots::Column::SpotId.eq(spot_id))
            .one(&tx)
            .await?
            .and_then(|(_, s)| s);
        match visitor_spot {
            Some(s) => {
                let mut visitor_spot: visitor_spots::ActiveModel = s.into();
                visitor_spot.last_scanned_at = Set(now);
                let res = visitor_spot.update(&tx).await;
                tx.commit().await?;

                res.to_is_updated()
            }
            None => {
                let visitor_spot = visitor_spots::ActiveModel {
                    visitor_id: Set(visitor_id),
                    spot_id: Set(spot_id),
                    last_scanned_at: Set(now),
                    ..Default::default()
                };
                let res = visitor_spot.insert(&tx).await;
                tx.commit().await?;

                res.to_is_updated()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::*;

    use crate::TestingSeaOrm;

    use super::*;

    impl TestingSeaOrm {
        async fn make_test_spot(&self, event_id: i32) -> EventSpot {
            let spot = crate::entity::event_spots::ActiveModel {
                event_id: Set(event_id),
                spot_id: Set(Id::new().dty()),
                name: Set("test".into()),
                is_pick: Set(false),
                bonus: Set(false),
                hw_id: Set("000002B920".into()),
                service_uuid: Set("CD:10:41:68:73:D6".into()),
                ..Default::default()
            }
            .insert(self.orm().con())
            .await
            .unwrap();

            to_model(spot).unwrap()
        }
    }

    #[test_log::test(tokio::test)]
    async fn test_list() {
        async fn test(q: u8) {
            let orm = TestingSeaOrm::new().await;
            let event = orm.make_test_event().await;
            let mut spots = Vec::new();
            for _ in 0..q {
                spots.push(orm.make_test_spot(event.id).await);
            }

            let res = SpotRepository::list(orm.orm(), event.id).await.unwrap();

            self::assert_eq!(res, spots);
        }

        test(1).await;
        test(2).await;
        test(3).await;
    }

    #[test_log::test(tokio::test)]
    async fn test_get_by_beacon() {
        let orm = TestingSeaOrm::new().await;
        let event = orm.make_test_event().await;
        let spot = orm.make_test_spot(event.id).await;

        let res = SpotRepository::get_by_beacon(orm.orm(), event.id, spot.hw_id.clone())
            .await
            .unwrap()
            .unwrap();

        self::assert_eq!(res, spot);
    }

    #[test_log::test(tokio::test)]
    async fn test_get_by_qr() {
        let orm = TestingSeaOrm::new().await;
        let event = orm.make_test_event().await;
        let spot = orm.make_test_spot(event.id).await;

        let res = SpotRepository::get_by_qr(orm.orm(), event.id, spot.spot_id)
            .await
            .unwrap()
            .unwrap();

        self::assert_eq!(res, spot);
    }

    #[test_log::test(tokio::test)]
    async fn test_update() {
        let orm = TestingSeaOrm::new().await;
        let event = orm.make_test_event().await;
        let spot = orm.make_test_spot(event.id).await;

        let res = SpotRepository::update(orm.orm(), event.id, spot.spot_id, "test2".into(), true)
            .await
            .unwrap();

        self::assert_eq!(
            res,
            Some(EventSpot {
                id: spot.id,
                spot_id: spot.spot_id,
                name: "test2".into(),
                is_pick: true,
                bonus: false,
                hw_id: spot.hw_id,
                service_uuid: "CD:10:41:68:73:D6".into(),
            })
        );
    }

    #[test_log::test(tokio::test)]
    async fn test_set_get_bonus() {
        let orm = TestingSeaOrm::new().await;
        let event = orm.make_test_event().await;
        let spot = orm.make_test_spot(event.id).await;

        let _ = SpotRepository::set_bonus_state(orm.orm(), event.id, spot.spot_id, true)
            .await
            .unwrap();
        let res = SpotRepository::get_bonus_state(orm.orm(), event.id, spot.spot_id)
            .await
            .unwrap();

        self::assert_eq!(res, true);
    }
}
