use async_trait::async_trait;
use repaint_server_model::event_spot::{EventSpot, IBeacon};
use repaint_server_model::id::Id;
use repaint_server_usecase::infra::repo::{IsUpdated, SpotRepository};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, TransactionTrait};

use crate::entity::{event_spots, events};
use crate::ty::string::ToDatabaseType;
use crate::{Error, SeaOrm};

use super::IsUpdatedExt;

pub fn to_model(m: event_spots::Model) -> Result<EventSpot, Error> {
    Ok(EventSpot {
        spot_id: m.spot_id.model(),
        name: m.name,
        is_pick: m.is_pick,
        bonus: m.bonus,
        i_beacon: IBeacon {
            major: m.major,
            minor: m.minor,
            beacon_uuid: m.beacon_uuid,
        },
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
        major: i16,
        minor: i16,
        beacon_uuid: String,
        hw_id: String,
        service_uuid: String,
    ) -> Result<EventSpot, Self::Error> {
        let spot = event_spots::ActiveModel {
            event_id: Set(event_id),
            spot_id: Set(Id::new().dty()),
            name: Set(name),
            is_pick: Set(false),
            bonus: Set(false),
            major: Set(major),
            minor: Set(minor),
            beacon_uuid: Set(beacon_uuid),
            hw_id: Set(hw_id),
            service_uuid: Set(service_uuid),
            ..Default::default()
        }
        .insert(self.con())
        .await?;

        to_model(spot)
    }

    async fn list(&self, event_id: i32) -> Result<Vec<EventSpot>, Self::Error> {
        events::Entity::find_by_id(event_id)
            .find_with_related(event_spots::Entity)
            .all(self.con())
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

    async fn update(
        &self,
        event_id: i32,
        spot_id: Id<EventSpot>,
        name: String,
        is_pick: bool,
    ) -> Result<EventSpot, Self::Error> {
        let mut spot: event_spots::ActiveModel = events::Entity::find_by_id(event_id)
            .find_with_related(event_spots::Entity)
            .all(self.con())
            .await?
            .into_iter()
            .map(|(_, s)| s)
            .flatten()
            .find(|s| s.spot_id == spot_id.dty())
            .unwrap()
            .into();
        spot.name = Set(name);
        spot.is_pick = Set(is_pick);
        let spot = spot.update(self.con()).await?;

        to_model(spot)
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
            .unwrap();
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
            .map(to_model)
            .transpose()
            .and_then(|s| Ok(s.unwrap().bonus))
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
            .unwrap()
            .into();
        spot.bonus = Set(is_bonus);
        let res = spot.update(&tx).await;
        tx.commit().await?;

        res.to_is_updated()
    }
}
