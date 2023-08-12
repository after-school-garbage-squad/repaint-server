use async_trait::async_trait;
use email_address::EmailAddress;
use repaint_server_model::admin::Admin;
use repaint_server_model::event::{Contact, Event};
use repaint_server_model::event_beacon::EventBeacon;
use repaint_server_model::event_image::Image as EventImage;
use repaint_server_model::event_spot::EventSpot;
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_model::visitor_image::{CurrentImage, Image as VisitorImage};
use repaint_server_model::{AsyncSafe, StaticError};

use crate::model::event::{CreateEventResponse, EventResponse, UpdateEventResponse};
use crate::model::spot::{SpotResponse, TrafficStatus};
use crate::model::visitor::{VisitorIdentification, VisitorResponse};

#[async_trait]
pub trait SpotRepository: AsyncSafe {
    type Error: StaticError;

    async fn register_spot(
        &self,
        event_id: Id<Event>,
        name: String,
        beacon_data: EventBeacon,
    ) -> Result<SpotResponse, Self::Error>;

    async fn check_status_by_beacon(
        &self,
        event_id: Id<Event>,
        beacon_data: EventBeacon,
    ) -> Result<Option<SpotResponse>, Self::Error>;

    async fn check_status_by_qr(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<Option<SpotResponse>, Self::Error>;

    async fn list_spot(&self, event_id: Id<Event>) -> Result<Vec<SpotResponse>, Self::Error>;

    async fn update_spot(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
        name: String,
        is_pick: bool,
    ) -> Result<SpotResponse, Self::Error>;

    async fn delete_spot(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<IsUpdated, Self::Error>;
}

#[async_trait]
pub trait EventRepository: AsyncSafe {
    type Error: StaticError;

    async fn create_event(
        &self,
        name: String,
        hp_url: String,
        contact: Contact,
    ) -> Result<CreateEventResponse, Self::Error>;

    async fn list_event(&self, admin_id: Id<Admin>) -> Result<Vec<EventResponse>, Self::Error>;

    async fn update_event(
        &self,
        event_id: Id<Event>,
        name: String,
        hp_url: String,
        contact: Contact,
    ) -> Result<UpdateEventResponse, Self::Error>;

    async fn delete_event(&self, event_id: Id<Event>) -> Result<IsUpdated, Self::Error>;

    async fn add_operator(
        &self,
        event_id: Id<Event>,
        email: EmailAddress,
    ) -> Result<(), Self::Error>;

    async fn controll_notification(
        &self,
        event_id: Id<Event>,
        content: String,
    ) -> Result<(), Self::Error>;

    async fn add_default_image(
        &self,
        event_id: Id<Event>,
        image: String, //FIXME
    ) -> Result<(), Self::Error>;

    async fn delete_default_image(
        &self,
        event_id: Id<Event>,
        image_id: Id<EventImage>,
    ) -> Result<IsUpdated, Self::Error>;

    async fn check_visitor_exist(
        &self,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
    ) -> Result<Option<VisitorResponse>, Self::Error>;

    async fn upload_visitor_image(
        &self,
        event_id: Id<Event>,
        image: String, //FIXME
    ) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait TrafficRepository: AsyncSafe {
    type Error: StaticError;

    async fn get_traffic_status(
        &self,
        event_id: Id<Event>,
    ) -> Result<Vec<TrafficStatus>, Self::Error>;

    async fn controll_traffic(
        &self,
        event_id: Id<Event>,
        from: Id<EventSpot>,
        to: Id<EventSpot>,
    ) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait VisitorRepository: AsyncSafe {
    type Error: StaticError;

    async fn join_event(
        &self,
        event_id: Id<Event>,
        registration_id: String,
    ) -> Result<(EventResponse, VisitorResponse), Self::Error>;

    async fn initialize_visitor(
        &self,
        visitor_identification: VisitorIdentification,
    ) -> Result<(EventResponse, VisitorResponse), Self::Error>;

    async fn publish_qr(
        &self,
        visitor_identification: VisitorIdentification,
    ) -> Result<String, Self::Error>; //FIXME

    async fn controll_notification(
        &self,
        visitor_identification: VisitorIdentification,
    ) -> Result<IsUpdated, Self::Error>;

    async fn delete_visitor(
        &self,
        visitor_identification: VisitorIdentification,
    ) -> Result<IsUpdated, Self::Error>;

    async fn list_image(
        &self,
        visitor_identification: VisitorIdentification,
    ) -> Result<Vec<Id<VisitorImage>>, Self::Error>;

    async fn set_current_image(
        &self,
        visitor_identification: VisitorIdentification,
        image_id: Id<VisitorImage>,
    ) -> Result<IsUpdated, Self::Error>;

    async fn get_current_image(
        &self,
        visitor_identification: VisitorIdentification,
    ) -> Result<Id<CurrentImage>, Self::Error>;

    async fn drop_palette(
        &self,
        visitor_identification: VisitorIdentification,
        beacon: EventBeacon,
    ) -> Result<(), Self::Error>;

    async fn pick_palette(
        &self,
        visitor_identification: VisitorIdentification,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Self::Error>;
}

#[derive(Debug)]
#[must_use]
pub struct IsUpdated(pub bool);

impl IsUpdated {
    pub fn ignore(self) {}

    pub fn ok_or<E>(self, e: E) -> Result<(), E> {
        self.ok_or_else(|| e)
    }

    pub fn ok_or_else<F, E>(self, f: F) -> Result<(), E>
    where
        F: FnOnce() -> E,
    {
        if self.0 {
            Ok(())
        } else {
            Err(f())
        }
    }
}
