use async_trait::async_trait;
use email_address::EmailAddress;
use repaint_server_model::event::Event;
use repaint_server_model::event_spot::EventSpot;
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_model::{AsyncSafe, StaticError};

#[async_trait]
pub trait Firestore: AsyncSafe {
    type Error: StaticError;

    async fn subscribe_palette(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
        palette_id: i32,
    ) -> Result<(), Self::Error>;

    async fn subscribe_palettes(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
        palettes_length: Vec<i32>,
    ) -> Result<(), Self::Error>;

    async fn get_palette(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<i32, Self::Error>;

    async fn get_palettes(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<Vec<i32>, Self::Error>;

    async fn delete_spot(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Self::Error>;

    async fn subscribe_visitor(
        &self,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
        spot_id: Id<EventSpot>,
    ) -> Result<(), Self::Error>;

    async fn get_visitors(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
    ) -> Result<Vec<Id<Visitor>>, Self::Error>;

    async fn set_email(
        &self,
        event_id: Id<Event>,
        token: String,
        email: EmailAddress,
    ) -> Result<(), Self::Error>;

    async fn get_email(
        &self,
        event_id: Id<Event>,
        token: String,
    ) -> Result<Option<EmailAddress>, Self::Error>;

    async fn subscribe_visitor_log(
        &self,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
        spot_id: Id<EventSpot>,
        palettes_length: usize,
        took_photo: bool,
    ) -> Result<(), Self::Error>;

    async fn subscribe_spot_log(
        &self,
        event_id: Id<Event>,
        spot_id: Id<EventSpot>,
        head_count: usize,
    ) -> Result<(), Self::Error>;

    async fn subscribe_traffic_log(
        &self,
        event_id: Id<Event>,
        from: Id<EventSpot>,
        to: Id<EventSpot>,
    ) -> Result<(), Self::Error>;

    async fn subscribe_register_log(
        &self,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
    ) -> Result<(), Self::Error>;

    async fn subscribe_initialize_log(
        &self,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
    ) -> Result<(), Self::Error>;

    async fn delete(&self, event_id: Id<Event>) -> Result<(), Self::Error>;
}
