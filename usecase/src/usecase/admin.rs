use async_trait::async_trait;
use email_address::EmailAddress;
use futures::future::join_all;
use rand::distributions::Alphanumeric;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use repaint_server_model::event::{Contact, Event};
use repaint_server_model::event_image::Image as EventImage;
use repaint_server_model::event_spot::EventSpot;
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_model::visitor_image::Image as VisitorImage;
use repaint_server_model::AsyncSafe;
use teloc::inject;

use crate::infra::email::EmailSender;
use crate::infra::fcm::FirebaseCloudMessaging;
use crate::infra::firestore::Firestore;
use crate::infra::repo::{
    AdminRepository, EventRepository, ImageRepository, SpotRepository, VisitorRepository,
};
use crate::model::event::{CreateEventResponse, EventResponse, UpdateEventResponse};
use crate::model::spot::{Beacon, SpotResponse, TrafficStatus};
use crate::usecase::error::Error;

#[async_trait]
pub trait AdminUsecase: AsyncSafe {
    async fn create_event(
        &self,
        subject: String,
        name: String,
        hp_url: String,
        contact: Contact,
    ) -> Result<CreateEventResponse, Error>;

    async fn delete_event(&self, subject: String, event_id: Id<Event>) -> Result<(), Error>;

    async fn list_event(&self, subject: String) -> Result<Vec<EventResponse>, Error>;

    async fn update_event(
        &self,
        subject: String,
        event_id: Id<Event>,
        name: String,
        hp_url: String,
        contact: Contact,
    ) -> Result<UpdateEventResponse, Error>;

    async fn add_default_image(
        &self,
        subject: String,
        event_id: Id<Event>,
        image_id: Id<EventImage>,
    ) -> Result<(), Error>;

    async fn delete_default_image(
        &self,
        subject: String,
        event_id: Id<Event>,
        image_id: Id<EventImage>,
    ) -> Result<(), Error>;

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

    async fn check_visitor_image_exist(
        &self,
        subject: String,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
    ) -> Result<Option<Id<VisitorImage>>, Error>;

    async fn upload_visitor_image(
        &self,
        subject: String,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
        image_id: Id<VisitorImage>,
    ) -> Result<(), Error>;

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

    async fn add_admin(&self, subject: String) -> Result<(), Error>;

    async fn send_email(
        &self,
        subject: String,
        event_id: Id<Event>,
        email: EmailAddress,
    ) -> Result<(), Error>;

    async fn add_operator(&self, subject: String, token: String) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct AdminUsecaseImpl<R> {
    repo: R,
}

#[inject]
impl<R> AdminUsecaseImpl<R>
where
    R: EventRepository
        + SpotRepository
        + ImageRepository
        + VisitorRepository
        + AdminRepository
        + Firestore
        + EmailSender
        + FirebaseCloudMessaging,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R> AdminUsecase for AdminUsecaseImpl<R>
where
    R: EventRepository
        + SpotRepository
        + ImageRepository
        + VisitorRepository
        + AdminRepository
        + Firestore
        + EmailSender
        + FirebaseCloudMessaging,
{
    async fn create_event(
        &self,
        subject: String,
        name: String,
        hp_url: String,
        contact: Contact,
    ) -> Result<CreateEventResponse, Error> {
        let admin = AdminRepository::get(&self.repo, subject)
            .await?
            .ok_or(Error::UnAuthorized)?;

        if name.chars().count() > 32 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 32 chars", name),
            });
        }

        if hp_url.chars().count() > 2083 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 2083 chars", hp_url),
            });
        }

        if contact.name.chars().count() > 32 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 32 chars", contact.name),
            });
        }

        if contact.email.as_str().chars().count() > 80 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 80 chars", contact.email),
            });
        }

        if contact.phone.chars().count() > 11 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 11 chars", contact.phone),
            });
        }

        let event = EventRepository::create(&self.repo, name, hp_url, contact).await?;

        let _ = AdminRepository::update(&self.repo, admin.id, event.id).await?;

        Ok(CreateEventResponse {
            event_id: event.event_id,
            name: event.name,
            hp_url: event.hp_url,
            contact: event.contact,
        })
    }

    async fn delete_event(&self, subject: String, event_id: Id<Event>) -> Result<(), Error> {
        let event = EventRepository::get_event_belong_to_subject(&self.repo, subject, event_id)
            .await?
            .ok_or(Error::UnAuthorized)?;

        let _ = EventRepository::delete(&self.repo, event.id).await?;
        Firestore::delete(&self.repo, event.event_id).await?;

        Ok(())
    }

    async fn list_event(&self, subject: String) -> Result<Vec<EventResponse>, Error> {
        let events = EventRepository::list(&self.repo, subject).await?;

        let s = events
            .iter()
            .map(|e| SpotRepository::list(&self.repo, e.id));
        let spots = join_all(s)
            .await
            .into_iter()
            .collect::<Result<Vec<Vec<_>>, _>>()?;

        let i = events
            .iter()
            .map(|e| ImageRepository::list_default_image(&self.repo, e.id));
        let images = join_all(i)
            .await
            .into_iter()
            .collect::<Result<Vec<Vec<_>>, _>>()?;

        Ok(events
            .into_iter()
            .zip(spots)
            .zip(images)
            .map(|((e, s), i)| EventResponse {
                event_id: e.event_id,
                name: e.name,
                hp_url: e.hp_url,
                contact: e.contact,
                spots: s,
                images: i,
            })
            .collect())
    }

    async fn update_event(
        &self,
        subject: String,
        event_id: Id<Event>,
        name: String,
        hp_url: String,
        contact: Contact,
    ) -> Result<UpdateEventResponse, Error> {
        let event = EventRepository::get_event_belong_to_subject(&self.repo, subject, event_id)
            .await?
            .ok_or(Error::UnAuthorized)?;

        if name.chars().count() > 32 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 32 chars", name),
            });
        }

        if hp_url.chars().count() > 2083 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 2083 chars", hp_url),
            });
        }

        if contact.name.chars().count() > 32 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 32 chars", contact.name),
            });
        }

        if contact.email.as_str().chars().count() > 80 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 80 chars", contact.email),
            });
        }

        if contact.phone.chars().count() > 11 {
            return Err(Error::BadRequest {
                message: format!("{} is longer than 11 chars", contact.phone),
            });
        }

        let event = EventRepository::update(&self.repo, event.id, name, hp_url, contact).await?;

        Ok(UpdateEventResponse {
            event_id: event.event_id,
            name: event.name,
            hp_url: event.hp_url,
            contact: event.contact,
        })
    }

    async fn add_default_image(
        &self,
        subject: String,
        event_id: Id<Event>,
        image_id: Id<EventImage>,
    ) -> Result<(), Error> {
        let event = EventRepository::get_event_belong_to_subject(&self.repo, subject, event_id)
            .await?
            .ok_or(Error::UnAuthorized)?;

        let _ = ImageRepository::add_default_image(&self.repo, event.id, image_id).await?;

        Ok(())
    }

    async fn delete_default_image(
        &self,
        subject: String,
        event_id: Id<Event>,
        image_id: Id<EventImage>,
    ) -> Result<(), Error> {
        let event = EventRepository::get_event_belong_to_subject(&self.repo, subject, event_id)
            .await?
            .ok_or(Error::UnAuthorized)?;

        let _ = ImageRepository::delete_default_image(&self.repo, event.id, image_id).await?;

        Ok(())
    }

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
            beacon_data.i_beacon.major,
            beacon_data.i_beacon.minor,
            beacon_data.i_beacon.beacon_uuid,
            beacon_data.hw_id,
            beacon_data.service_uuid,
        )
        .await?;

        Ok(SpotResponse {
            spot_id: spot.spot_id,
            name: spot.name,
            beacon: Beacon {
                i_beacon: spot.i_beacon,
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
                i_beacon: spot.i_beacon,
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
                i_beacon: spot.i_beacon,
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
                    i_beacon: s.i_beacon,
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

        let spot = SpotRepository::update(&self.repo, event.id, spot_id, name, is_pick).await?;

        Ok(SpotResponse {
            spot_id: spot.spot_id,
            name: spot.name,
            beacon: Beacon {
                i_beacon: spot.i_beacon,
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
        Firestore::delete_spot(&self.repo, event.event_id, spot_id).await?;

        Ok(())
    }

    async fn check_visitor_image_exist(
        &self,
        subject: String,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
    ) -> Result<Option<Id<VisitorImage>>, Error> {
        let event = EventRepository::get_event_belong_to_subject(&self.repo, subject, event_id)
            .await?
            .ok_or(Error::UnAuthorized)?;

        let visitor = VisitorRepository::get(&self.repo, event.id, visitor_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} aren't exist", visitor_id),
            })?;

        let image = ImageRepository::get_visitor_image(&self.repo, visitor.id).await?;

        Ok(image)
    }

    async fn upload_visitor_image(
        &self,
        subject: String,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
        image_id: Id<VisitorImage>,
    ) -> Result<(), Error> {
        let event = EventRepository::get_event_belong_to_subject(&self.repo, subject, event_id)
            .await?
            .ok_or(Error::UnAuthorized)?;

        let visitor = VisitorRepository::get(&self.repo, event.id, visitor_id)
            .await?
            .ok_or(Error::BadRequest {
                message: format!("{} aren't exist", visitor_id),
            })?;

        let _ = ImageRepository::upload_visitor_image(&self.repo, visitor.id, image_id).await?;

        Ok(())
    }

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
            .map(|s| Firestore::get_visitors(&self.repo, event_id, s.spot_id));
        let visitors = join_all(s)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        let s = spots
            .iter()
            .zip(visitors.clone())
            .map(|(s, v)| Firestore::subscribe_spot_log(&self.repo, event_id, s.spot_id, v.len()));
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

        let visitors_in_from = Firestore::get_visitors(&self.repo, event.event_id, from).await?;
        let visitors_in_to = Firestore::get_visitors(&self.repo, event.event_id, to).await?;

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

        let v = visitors
            .iter()
            .map(|&v| VisitorRepository::get(&self.repo, event.id, v));
        let visitors = join_all(v).await.into_iter().flatten().collect::<Vec<_>>();

        let m = visitors
            .into_iter()
            .flatten()
            .map(|v| FirebaseCloudMessaging::send(&self.repo, v.registration_id));
        join_all(m)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        let _ = SpotRepository::set_bonus_state(&self.repo, event.id, to, true).await?;

        Firestore::subscribe_traffic_log(&self.repo, event.event_id, from, to).await?;

        Ok(())
    }

    async fn add_admin(&self, subject: String) -> Result<(), Error> {
        let _ = AdminRepository::add(&self.repo, subject).await?;

        Ok(())
    }

    async fn send_email(
        &self,
        subject: String,
        event_id: Id<Event>,
        email: EmailAddress,
    ) -> Result<(), Error> {
        let event = EventRepository::get_event_belong_to_subject(&self.repo, subject, event_id)
            .await?
            .ok_or(Error::UnAuthorized)?;

        let token = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect::<String>();

        EmailSender::send(&self.repo, email.clone(), token.clone()).await?;

        Firestore::set_event_id(&self.repo, token, event.event_id).await?;

        Ok(())
    }

    async fn add_operator(&self, subject: String, token: String) -> Result<(), Error> {
        let admin = AdminRepository::get(&self.repo, subject)
            .await?
            .ok_or(Error::UnAuthorized)?;

        let event_id =
            Firestore::get_event_id(&self.repo, token)
                .await?
                .ok_or(Error::BadRequest {
                    message: "This token has already expired or is invalid.".to_string(),
                })?;

        let _ = AdminRepository::update(&self.repo, admin.id, event_id).await?;

        Ok(())
    }
}
