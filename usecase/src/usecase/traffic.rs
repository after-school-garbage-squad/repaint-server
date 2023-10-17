use async_trait::async_trait;
use futures::future::join_all;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use repaint_server_model::event::Event;
use repaint_server_model::event_spot::EventSpot;
use repaint_server_model::id::Id;
use repaint_server_model::AsyncSafe;
use teloc::inject;

use crate::infra::pubsub::GoogleCloudPubSub;
use crate::infra::repo::{
    EventRepository, SpotRepository, TrafficRepository, TransactionRepository, VisitorRepository,
};
use crate::model::traffic::{GetTrafficStatusResponse, TrafficStatus};
use crate::usecase::error::Error;

#[async_trait]
pub trait TrafficUsecase: AsyncSafe {
    async fn get_traffic_status(
        &self,
        subject: String,
        event_id: Id<Event>,
    ) -> Result<GetTrafficStatusResponse, Error>;

    async fn enable_bonus(
        &self,
        subject: String,
        event_id: Id<Event>,
        from: Id<EventSpot>,
        to: Id<EventSpot>,
    ) -> Result<(), Error>;

    async fn disable_bonus(
        &self,
        subject: String,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct TrafficUsecaseImpl<R, P> {
    repo: R,
    pubsub: P,
}

#[inject]
impl<R, P> TrafficUsecaseImpl<R, P>
where
    R: EventRepository
        + SpotRepository
        + VisitorRepository
        + TrafficRepository
        + TransactionRepository,
    P: GoogleCloudPubSub,
{
    pub fn new(repo: R, pubsub: P) -> Self {
        Self { repo, pubsub }
    }
}

#[async_trait]
impl<R, P> TrafficUsecase for TrafficUsecaseImpl<R, P>
where
    R: EventRepository
        + SpotRepository
        + VisitorRepository
        + TrafficRepository
        + TransactionRepository,
    P: GoogleCloudPubSub,
{
    async fn get_traffic_status(
        &self,
        subject: String,
        event_id: Id<Event>,
    ) -> Result<GetTrafficStatusResponse, Error> {
        let tx = TransactionRepository::begin_transaction(&self.repo).await?;
        let event =
            EventRepository::get_event_belong_to_subject(&self.repo, &tx, subject, event_id)
                .await?
                .ok_or(Error::UnAuthorized)?;

        let spots = SpotRepository::list(&self.repo, &tx, event.id).await?;

        let s = spots
            .iter()
            .map(|s| VisitorRepository::get_visitors(&self.repo, &tx, s.id));
        let visitors = join_all(s)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;
        let _ = tx.commit().await;

        Ok(GetTrafficStatusResponse {
            traffics: spots
                .into_iter()
                .zip(visitors)
                .map(|(s, v)| TrafficStatus {
                    spot_id: s.spot_id,
                    head_count: v.len(),
                })
                .collect(),
        })
    }

    async fn enable_bonus(
        &self,
        subject: String,
        event_id: Id<Event>,
        from: Id<EventSpot>,
        to: Id<EventSpot>,
    ) -> Result<(), Error> {
        let tx = TransactionRepository::begin_transaction(&self.repo).await?;
        let event =
            EventRepository::get_event_belong_to_subject(&self.repo, &tx, subject, event_id)
                .await?
                .ok_or(Error::UnAuthorized)?;
        let size = TrafficRepository::size(&self.repo, &tx).await?;
        if size >= 4 {
            let id = TrafficRepository::pop(&self.repo, &tx)
                .await?
                .ok_or(Error::BadRequest {
                    message: "traffic queue is empty".to_string(),
                })?;
            let spot = SpotRepository::get_by_id(&self.repo, &tx, id)
                .await?
                .ok_or(Error::BadRequest {
                    message: format!("{} is invalid id", id),
                })?;
            let _ = SpotRepository::set_bonus_state(&self.repo, &tx, event.id, spot.spot_id, false)
                .await?;
        }
        let from = SpotRepository::get_by_qr(&self.repo, &tx, event.id, from)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", from),
            })?;
        let to = SpotRepository::get_by_qr(&self.repo, &tx, event.id, to)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", to),
            })?;
        let visitors_in_from = VisitorRepository::get_visitors(&self.repo, &tx, from.id).await?;
        let visitors_in_to = VisitorRepository::get_visitors(&self.repo, &tx, to.id).await?;
        let mut rng = {
            let rng = rand::thread_rng();
            StdRng::from_rng(rng).unwrap()
        };
        let visitors = visitors_in_from
            .choose_multiple(&mut rng, (visitors_in_from.len() + 1) / 2)
            .cloned()
            .collect::<Vec<_>>();
        let spot = SpotRepository::get_by_qr(&self.repo, &tx, event.id, to.spot_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("spot isn't found"),
            })?;
        let v = visitors
            .iter()
            .map(|&v| VisitorRepository::get_by_id(&self.repo, &tx, v));
        let visitors = join_all(v).await.into_iter().flatten().collect::<Vec<_>>();
        let m = visitors.into_iter().flatten().map(|v| {
            self.pubsub
                .publish_notification(v.registration_id, spot.name.clone())
        });
        join_all(m)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;
        let _ =
            SpotRepository::set_bonus_state(&self.repo, &tx, event.id, to.spot_id, true).await?;
        let _ = TrafficRepository::push(
            &self.repo,
            &tx,
            to.id,
            visitors_in_from.len(),
            visitors_in_to.len(),
        )
        .await?;
        let _ = tx.commit().await?;

        Ok(())
    }

    async fn disable_bonus(
        &self,
        subject: String,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Error> {
        let tx = TransactionRepository::begin_transaction(&self.repo).await?;
        let event =
            EventRepository::get_event_belong_to_subject(&self.repo, &tx, subject, event_id)
                .await?
                .ok_or(Error::UnAuthorized)?;
        let spot = SpotRepository::get_by_qr(&self.repo, &tx, event.id, spot_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", spot_id),
            })?;
        let _ = SpotRepository::set_bonus_state(&self.repo, &tx, event.id, spot_id, false).await?;
        let _ = TrafficRepository::remove(&self.repo, &tx, spot.id).await?;
        let _ = tx.commit().await?;

        Ok(())
    }
}
