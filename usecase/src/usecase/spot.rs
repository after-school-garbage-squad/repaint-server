use std::cmp::max;
use std::str::FromStr;

use async_trait::async_trait;
use chrono::{Duration, Utc};
use futures::future::join_all;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use repaint_server_model::event::Event;
use repaint_server_model::event_spot::EventSpot;
use repaint_server_model::id::Id;
use repaint_server_model::visitor_image::CurrentImage;
use repaint_server_model::visitor_image::Image as VisitorImage;
use repaint_server_model::AsyncSafe;
use repaint_server_util::envvar;
use teloc::inject;

use crate::infra::firestore::Firestore;
use crate::infra::pubsub::GoogleCloudPubSub;
use crate::infra::repo::TrafficRepository;
use crate::infra::repo::{
    EventRepository, ImageRepository, PaletteRepository, SpotRepository, VisitorRepository,
};
use crate::model::spot::ScannedResponse;
use crate::model::spot::{Beacon, SpotResponse};
use crate::model::visitor::VisitorIdentification;
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

    async fn scanned(
        &self,
        visitor_identification: VisitorIdentification,
        hw_id: String,
    ) -> Result<ScannedResponse, Error>;
}

#[derive(Debug, Clone)]
pub struct SpotUsecaseImpl<R, F, P> {
    repo: R,
    firestore: F,
    pubsub: P,
}

#[inject]
impl<R, F, P> SpotUsecaseImpl<R, F, P>
where
    R: SpotRepository
        + EventRepository
        + VisitorRepository
        + ImageRepository
        + PaletteRepository
        + TrafficRepository,
    F: Firestore,
    P: GoogleCloudPubSub,
{
    pub fn new(repo: R, firestore: F, pubsub: P) -> Self {
        Self {
            repo,
            firestore,
            pubsub,
        }
    }
}

#[async_trait]
impl<R, F, P> SpotUsecase for SpotUsecaseImpl<R, F, P>
where
    R: SpotRepository
        + EventRepository
        + VisitorRepository
        + ImageRepository
        + PaletteRepository
        + TrafficRepository,
    F: Firestore,
    P: GoogleCloudPubSub,
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
        let Some(mut palettes) = PaletteRepository::get_all(&self.repo, event.id).await? else {
            unreachable!("palettes is not set")
        };
        let p = palettes.clone().into_iter().map(|palette| {
            self.firestore
                .subscribe_palette(event.event_id, spot.spot_id, palette)
        });
        let _ = join_all(p)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;
        for palette in palettes.iter_mut() {
            *palette += 1;
        }
        let _ = PaletteRepository::set_all(&self.repo, event.id, palettes).await?;

        Ok(SpotResponse {
            spot_id: spot.spot_id,
            name: spot.name,
            beacon: Beacon {
                hw_id: spot.hw_id,
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
        let spot = SpotRepository::get_by_beacon(&self.repo, event.id, hw_id.clone())
            .await?
            .ok_or(Error::BadRequest {
                message: format!("No spots associated with {} have been registered", hw_id),
            })?;

        Ok(Some(SpotResponse {
            spot_id: spot.spot_id,
            name: spot.name,
            beacon: Beacon {
                hw_id: spot.hw_id,
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

        Ok(Some(SpotResponse {
            spot_id: spot.spot_id,
            name: spot.name,
            beacon: Beacon {
                hw_id: spot.hw_id,
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

        Ok(SpotResponse {
            spot_id: spot.spot_id,
            name: spot.name,
            beacon: Beacon {
                hw_id: spot.hw_id,
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

        Ok(())
    }

    async fn scanned(
        &self,
        visitor_identification: VisitorIdentification,
        hw_id: String,
    ) -> Result<ScannedResponse, Error> {
        let now = Utc::now().naive_utc();
        let event = EventRepository::get(&self.repo, visitor_identification.event_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} is invalid id", visitor_identification.event_id),
            })?;
        let visitor =
            VisitorRepository::get(&self.repo, event.id, visitor_identification.visitor_id)
                .await?
                .ok_or(Error::BadRequest {
                    message: format!("{} is invalid id", visitor_identification.visitor_id),
                })?;
        let spot = SpotRepository::get_by_beacon(&self.repo, event.id, hw_id.clone())
            .await?
            .ok_or(Error::BadRequest {
                message: format!("No spots associated with {} have been registered", hw_id),
            })?;
        let last_droped = VisitorRepository::get_last_droped_at(&self.repo, visitor.id).await?;
        let is_bonus = SpotRepository::get_bonus_state(&self.repo, event.id, spot.spot_id).await?;
        if last_droped.is_none()
            || now - last_droped.unwrap()
                >= Duration::seconds(if is_bonus {
                    envvar("BONUS_DROP_INTERVAL", 180)
                } else {
                    envvar("DROP_INTERVAL", 300)
                })
        {
            if is_bonus {
                let Some(timestamp) = TrafficRepository::get_timestamp(&self.repo, spot.id).await?
                else {
                    unreachable!("traffic timestamp is not set")
                };
                let visitors_now = VisitorRepository::get_visitors(&self.repo, spot.id).await?;
                let Some(visitors_start) = TrafficRepository::get_hc(&self.repo, spot.id).await?
                else {
                    unreachable!("traffic hc is not set")
                };
                if Utc::now() - timestamp.and_utc()
                    >= Duration::seconds(envvar("BONUS_TIMEOUT", 1800))
                    || visitors_now.len()
                        > max(
                            ((visitors_start.hc_from as f32) * 0.4) as usize,
                            ((max(visitors_start.hc_to, 5) as f32) * 1.5) as usize,
                        )
                {
                    let _ =
                        SpotRepository::set_bonus_state(&self.repo, event.id, spot.spot_id, false)
                            .await?;
                    let _ = TrafficRepository::remove(&self.repo, spot.id).await?;
                }
            }
            let palettes = PaletteRepository::get(&self.repo, visitor.id).await?;
            let image = match ImageRepository::get_current_image(&self.repo, visitor.id).await? {
                Some(i) => i,
                None => {
                    let default = ImageRepository::list_default_image(&self.repo, event.id).await?;
                    let current_image_id = default
                        .first()
                        .ok_or(Error::BadRequest {
                            message: "default image is empty".to_string(),
                        })?
                        .clone();

                    Id::<CurrentImage>::from_str(current_image_id.to_string().as_str())
                        .ok()
                        .ok_or(Error::BadRequest {
                            message: "failed to parse default image id to current image id"
                                .to_string(),
                        })?
                }
            };
            let image_id = Id::<VisitorImage>::from_str(image.to_string().as_str())?;
            let _ = self
                .pubsub
                .publish_merge_current_image(
                    event.event_id,
                    visitor.visitor_id,
                    image_id,
                    palettes.clone(),
                )
                .await?;
            let took_photo = ImageRepository::get_visitor_image(&self.repo, visitor.id)
                .await?
                .is_some();
            self.firestore
                .subscribe_visitor_log(
                    visitor_identification.event_id,
                    visitor_identification.visitor_id,
                    spot.spot_id,
                    palettes.len(),
                    took_photo,
                )
                .await?;
            let Some(mut palettes) = PaletteRepository::get_all(&self.repo, event.id).await? else {
                unreachable!("palettes is not set")
            };
            let visitor_palettes = PaletteRepository::get(&self.repo, visitor.id)
                .await?
                .into_iter()
                .collect::<Vec<_>>();
            let mut rng = {
                let rng = rand::thread_rng();
                StdRng::from_rng(rng).unwrap()
            };
            let p = self
                .firestore
                .get_palettes(visitor_identification.event_id, spot.spot_id)
                .await?;
            let palette = p.choose(&mut rng).ok_or(Error::NotFound)?;
            let drop_palette = visitor_palettes.choose(&mut rng).ok_or(Error::NotFound)?;
            if !visitor_palettes.contains(palette) {
                let _ = self
                    .firestore
                    .unsubscribe_palette(event.event_id, spot.spot_id, *palette)
                    .await?;
                palettes[*palette as usize] += 1;
                let _ = PaletteRepository::set(&self.repo, visitor.id, *palette).await?;
                let _ = PaletteRepository::set_all(&self.repo, event.id, palettes).await?;
                let _ = self
                    .firestore
                    .subscribe_palette(event.event_id, spot.spot_id, *drop_palette)
                    .await?;
            }
            let _ = VisitorRepository::set_last_droped_at(&self.repo, visitor.id, now).await?;
        }
        let _ = SpotRepository::scanned(&self.repo, visitor.id, spot.id, now).await?;

        Ok(ScannedResponse { is_bonus })
    }
}
