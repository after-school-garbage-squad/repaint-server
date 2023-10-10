use async_trait::async_trait;
use repaint_server_model::event::Event;
use repaint_server_model::event_spot::EventSpot;
use repaint_server_model::id::Id;
use repaint_server_model::AsyncSafe;
use teloc::inject;

use crate::infra::firestore::Firestore;
use crate::infra::repo::{EventRepository, SpotRepository};
use crate::model::spot::{Beacon, SpotResponse};
use crate::usecase::error::Error;

#[async_trait]
pub trait SpotUsecase: AsyncSafe {
    async fn register_spot(
        &self,
        subject: String,
        event_id: Id<Event>,
        name: String,
        beacon_data: Beacon,
    ) -> Result<SpotResponse, Error>;

    async fn check_status_by_beacon(
        &self,
        subject: String,
        event_id: Id<Event>,
        hw_id: String,
    ) -> Result<Option<SpotResponse>, Error>;

    async fn check_status_by_qr(
        &self,
        subject: String,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<Option<SpotResponse>, Error>;

    async fn list_spot(
        &self,
        subject: String,
        event_id: Id<Event>,
    ) -> Result<Vec<SpotResponse>, Error>;

    async fn update_spot(
        &self,
        subject: String,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
        name: String,
        is_pick: bool,
    ) -> Result<SpotResponse, Error>;

    async fn delete_spot(
        &self,
        subject: String,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct SpotUsecaseImpl<R, F> {
    repo: R,
    firestore: F,
}

#[inject]
impl<R, F> SpotUsecaseImpl<R, F>
where
    R: SpotRepository + EventRepository,
    F: Firestore,
{
    pub fn new(repo: R, firestore: F) -> Self {
        Self { repo, firestore }
    }
}

#[async_trait]
impl<R, F> SpotUsecase for SpotUsecaseImpl<R, F>
where
    R: SpotRepository + EventRepository,
    F: Firestore,
{
    async fn register_spot(
        &self,
        subject: String,
        event_id: Id<Event>,
        name: String,
        beacon_data: Beacon,
    ) -> Result<SpotResponse, Error> {
        let event = EventRepository::get_event_belong_to_subject(&self.repo, subject, event_id)
            .await?
            .ok_or(Error::UnAuthorized)?;

        if name.chars().count() > 32 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 32 chars", name),
            });
        }

        let spot = SpotRepository::register(
            &self.repo,
            event.id,
            name,
            beacon_data.hw_id,
            beacon_data.service_uuid,
        )
        .await?;
        let hw_id = match spot.hw_id.starts_with(format!("{}-", event.id).as_str()) {
            true => spot
                .hw_id
                .trim_start_matches(format!("{}-", event.id).as_str()),
            false => {
                return Err(Error::BadRequest {
                    message: format!("{} is invalid", spot.hw_id),
                })
            }
        };

        Ok(SpotResponse {
            spot_id: spot.spot_id,
            name: spot.name,
            beacon: Beacon {
                hw_id: hw_id.into(),
                service_uuid: spot.service_uuid,
            },
            is_pick: spot.is_pick,
            bonus: spot.bonus,
        })
    }

    async fn check_status_by_beacon(
        &self,
        subject: String,
        event_id: Id<Event>,
        hw_id: String,
    ) -> Result<Option<SpotResponse>, Error> {
        let event = EventRepository::get_event_belong_to_subject(&self.repo, subject, event_id)
            .await?
            .ok_or(Error::UnAuthorized)?;
        let h = format!("{}-{}", event.id, hw_id);
        let spot = SpotRepository::get_by_beacon(&self.repo, event.id, h)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("No spots associated with {} have been registered", hw_id),
            })?;

        Ok(Some(SpotResponse {
            spot_id: spot.spot_id,
            name: spot.name,
            beacon: Beacon {
                hw_id,
                service_uuid: spot.service_uuid,
            },
            is_pick: spot.is_pick,
            bonus: spot.bonus,
        }))
    }

    async fn check_status_by_qr(
        &self,
        subject: String,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<Option<SpotResponse>, Error> {
        let event = EventRepository::get_event_belong_to_subject(&self.repo, subject, event_id)
            .await?
            .ok_or(Error::UnAuthorized)?;

        let spot = SpotRepository::get_by_qr(&self.repo, event.id, spot_id)
            .await?
            .ok_or(Error::BadRequest {
                message: "This QR code is invalid.".to_string(),
            })?;
        let hw_id = match spot.hw_id.starts_with(format!("{}-", event.id).as_str()) {
            true => spot
                .hw_id
                .trim_start_matches(format!("{}-", event.id).as_str()),
            false => {
                return Err(Error::BadRequest {
                    message: format!("{} is invalid", spot.hw_id),
                })
            }
        };

        Ok(Some(SpotResponse {
            spot_id: spot.spot_id,
            name: spot.name,
            beacon: Beacon {
                hw_id: hw_id.into(),
                service_uuid: spot.service_uuid,
            },
            is_pick: spot.is_pick,
            bonus: spot.bonus,
        }))
    }

    async fn list_spot(
        &self,
        subject: String,
        event_id: Id<Event>,
    ) -> Result<Vec<SpotResponse>, Error> {
        let event = EventRepository::get_event_belong_to_subject(&self.repo, subject, event_id)
            .await?
            .ok_or(Error::UnAuthorized)?;

        let spots = SpotRepository::list(&self.repo, event.id).await?;

        Ok(spots
            .into_iter()
            .map(|s| SpotResponse {
                spot_id: s.spot_id,
                name: s.name,
                beacon: Beacon {
                    hw_id: s.hw_id,
                    service_uuid: s.service_uuid,
                },
                is_pick: s.is_pick,
                bonus: s.bonus,
            })
            .collect())
    }

    async fn update_spot(
        &self,
        subject: String,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
        name: String,
        is_pick: bool,
    ) -> Result<SpotResponse, Error> {
        let event = EventRepository::get_event_belong_to_subject(&self.repo, subject, event_id)
            .await?
            .ok_or(Error::UnAuthorized)?;

        if name.chars().count() > 32 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 32 chars", name),
            });
        }

        let Some(spot) =
            SpotRepository::update(&self.repo, event.id, spot_id, name, is_pick).await?
        else {
            return Err(Error::BadRequest {
                message: format!("{} is not found", spot_id),
            });
        };
        let hw_id = match spot.hw_id.starts_with(format!("{}-", event.id).as_str()) {
            true => spot
                .hw_id
                .trim_start_matches(format!("{}-", event.id).as_str()),
            false => {
                return Err(Error::BadRequest {
                    message: format!("{} is invalid", spot.hw_id),
                })
            }
        };

        Ok(SpotResponse {
            spot_id: spot.spot_id,
            name: spot.name,
            beacon: Beacon {
                hw_id: hw_id.into(),
                service_uuid: spot.service_uuid,
            },
            is_pick: spot.is_pick,
            bonus: spot.bonus,
        })
    }

    async fn delete_spot(
        &self,
        subject: String,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Error> {
        let event = EventRepository::get_event_belong_to_subject(&self.repo, subject, event_id)
            .await?
            .ok_or(Error::UnAuthorized)?;

        let _ = SpotRepository::delete(&self.repo, event.id, spot_id).await?;
        self.firestore.delete_spot(event.event_id, spot_id).await?;

        Ok(())
    }
}
