use async_trait::async_trait;
use repaint_server_model::event::Event;
use repaint_server_model::event_image::Image as EventImage;
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_model::visitor_image::Image as VisitorImage;
use repaint_server_model::{AsyncSafe, StaticError};

#[async_trait]
pub trait GoogleCloudPubSub: AsyncSafe {
    type Error: StaticError;

    async fn publish_event_image(
        &self,
        event_id: Id<Event>,
        image_id: Id<EventImage>,
    ) -> Result<(), Self::Error>;

    async fn publish_visitor_image(
        &self,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
        image_id: Id<VisitorImage>,
    ) -> Result<(), Self::Error>;
}
