use async_trait::async_trait;
use chrono::NaiveDateTime;
use repaint_server_usecase::infra::repo::{IsUpdated, TrafficRepository};
use repaint_server_usecase::model::traffic::HeadCountResponse;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, QueryFilter,
    QueryOrder, QuerySelect, Set, TransactionTrait,
};

use crate::entity::traffic_queues;
use crate::{Error, SeaOrm};

use super::IsUpdatedExt;

#[async_trait]
impl TrafficRepository for SeaOrm {
    type Error = Error;

    async fn size(&self, tx: &DatabaseTransaction) -> Result<usize, Self::Error> {
        let length = traffic_queues::Entity::find()
            .order_by_asc(traffic_queues::Column::Timestamp)
            .all(tx)
            .await?
            .len();

        Ok(length)
    }

    async fn push(
        &self,
        tx: &DatabaseTransaction,
        spot_id: i32,
        hc_from: usize,
        hc_to: usize,
    ) -> Result<IsUpdated, Self::Error> {
        traffic_queues::ActiveModel {
            spot_id: Set(spot_id),
            head_count_from: Set(hc_from as i32),
            head_count_to: Set(hc_to as i32),
            ..Default::default()
        }
        .insert(tx)
        .await
        .to_is_updated()
    }

    async fn pop(&self, txn: &DatabaseTransaction) -> Result<Option<i32>, Self::Error> {
        let tx = txn.begin().await?;
        let last = traffic_queues::Entity::find()
            .order_by_desc(traffic_queues::Column::Timestamp)
            .one(&tx)
            .await?;
        match last {
            Some(last) => {
                traffic_queues::Entity::delete_by_id(last.id)
                    .exec(&tx)
                    .await?;
                tx.commit().await?;

                Ok(Some(last.id))
            }
            None => {
                tx.commit().await?;

                Ok(None)
            }
        }
    }

    async fn remove(
        &self,
        txn: &DatabaseTransaction,
        spot_id: i32,
    ) -> Result<IsUpdated, Self::Error> {
        let tx = txn.begin().await?;
        let last = traffic_queues::Entity::find()
            .filter(traffic_queues::Column::SpotId.eq(spot_id))
            .order_by_desc(traffic_queues::Column::Timestamp)
            .one(&tx)
            .await?
            .ok_or(Error::SeaOrm(DbErr::RecordNotFound(format!(
                "traffic_queues doesn't find with {}",
                spot_id
            ))))?;
        let res = traffic_queues::Entity::delete_by_id(last.id)
            .exec(&tx)
            .await;
        tx.commit().await?;

        res.to_is_updated()
    }

    async fn get_timestamp(
        &self,
        tx: &DatabaseTransaction,
        spot_id: i32,
    ) -> Result<Option<NaiveDateTime>, Self::Error> {
        let last = traffic_queues::Entity::find()
            .filter(traffic_queues::Column::SpotId.eq(spot_id))
            .limit(1)
            .one(tx)
            .await?;

        Ok(last.map(|last| last.timestamp))
    }

    async fn get_hc(
        &self,
        tx: &DatabaseTransaction,
        spot_id: i32,
    ) -> Result<Option<HeadCountResponse>, Self::Error> {
        let last = traffic_queues::Entity::find()
            .filter(traffic_queues::Column::SpotId.eq(spot_id))
            .limit(1)
            .one(tx)
            .await?;

        Ok(last.map(|last| HeadCountResponse {
            hc_from: last.head_count_from,
            hc_to: last.head_count_to,
        }))
    }
}
