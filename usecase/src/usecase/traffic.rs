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

use crate::infra::fcm::FirebaseCloudMessaging;
use crate::infra::firestore::Firestore;
use crate::infra::repo::{EventRepository, SpotRepository, VisitorRepository};
use crate::model::traffic::TrafficStatus;
use crate::usecase::error::Error;

#[async_trait]
pub trait TrafficUsecase: AsyncSafe {
    async fn get_traffic_status(
        &self,
        subject: String,
        event_id: Id<Event>,
    ) -> Result<Vec<TrafficStatus>, Error>;

    async fn controll_traffic(
        &self,
        subject: String,
        event_id: Id<Event>,
        from: Id<EventSpot>,
        to: Id<EventSpot>,
    ) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct TrafficUsecaseImpl<R, F, C> {
    repo: R,
    firestore: F,
    fcm: C,
}

#[inject]
impl<R, F, C> TrafficUsecaseImpl<R, F, C>
where
    R: EventRepository + SpotRepository + VisitorRepository,
    F: Firestore,
    C: FirebaseCloudMessaging,
{
    pub fn new(repo: R, firestore: F, fcm: C) -> Self {
        Self {
            repo,
            firestore,
            fcm,
        }
    }
}

#[async_trait]
impl<R, F, C> TrafficUsecase for TrafficUsecaseImpl<R, F, C>
where
    R: EventRepository + SpotRepository + VisitorRepository,
    F: Firestore,
    C: FirebaseCloudMessaging,
{
    async fn get_traffic_status(
        &self,
        subject: String,
        event_id: Id<Event>,
    ) -> Result<Vec<TrafficStatus>, Error> {
        let event = EventRepository::get_event_belong_to_subject(&self.repo, subject, event_id)
            .await?
            .ok_or(Error::UnAuthorized)?;

        let spots = SpotRepository::list(&self.repo, event.id).await?;

        let s = spots
            .iter()
            .map(|s| self.firestore.get_visitors(event_id, s.spot_id));
        let visitors = join_all(s)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        let s = spots.iter().zip(visitors.clone()).map(|(s, v)| {
            self.firestore
                .subscribe_spot_log(event_id, s.spot_id, v.len())
        });
        join_all(s)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        Ok(spots
            .into_iter()
            .zip(visitors)
            .map(|(s, v)| TrafficStatus {
                spot_id: s.spot_id,
                head_count: v.len(),
            })
            .collect())
    }

    async fn controll_traffic(
        &self,
        subject: String,
        event_id: Id<Event>,
        from: Id<EventSpot>,
        to: Id<EventSpot>,
    ) -> Result<(), Error> {
        let event = EventRepository::get_event_belong_to_subject(&self.repo, subject, event_id)
            .await?
            .ok_or(Error::UnAuthorized)?;

        let visitors_in_from = self.firestore.get_visitors(event.event_id, from).await?;
        let visitors_in_to = self.firestore.get_visitors(event.event_id, to).await?;

        let mut rng = {
            let rng = rand::thread_rng();
            StdRng::from_rng(rng).unwrap()
        };

        let visitors = visitors_in_from
            .choose_multiple(
                &mut rng,
                (visitors_in_from.len() + visitors_in_to.len()) / 2,
            )
            .cloned()
            .collect::<Vec<_>>();

        let spot = SpotRepository::get_by_qr(&self.repo, event.id, to)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("spot isn't found"),
            })?;

        let v = visitors
            .iter()
            .map(|&v| VisitorRepository::get(&self.repo, event.id, v));
        let visitors = join_all(v).await.into_iter().flatten().collect::<Vec<_>>();

        let m = visitors
            .into_iter()
            .flatten()
            .map(|v| self.fcm.send(v.registration_id, spot.name.clone()));
        join_all(m)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        let _ = SpotRepository::set_bonus_state(&self.repo, event.id, to, true).await?;

        self.firestore
            .subscribe_traffic_log(event.event_id, from, to)
            .await?;

        Ok(())
    }
}
