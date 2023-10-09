use async_trait::async_trait;
use repaint_server_model::event::Event;
use repaint_server_model::event_image::Image as EventImage;
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_model::visitor_image::{CurrentImage, Image as VisitorImage};
use repaint_server_model::{AsyncSafe, StaticError};

use crate::model::otp::Token;

#[async_trait]
pub trait ImageOtp: AsyncSafe {
    type Error: StaticError;

    async fn verify_current(
        &self,
        event_id: Id<Event>,
        image_id: Id<CurrentImage>,
        visitor_id: Id<Visitor>,
    ) -> Result<Token, Self::Error>;

    async fn verify_event(
        &self,
        event_id: Id<Event>,
        image_id: Id<EventImage>,
    ) -> Result<Token, Self::Error>;

    async fn verify_gray(
        &self,
        event_id: Id<Event>,
        image_id: Id<VisitorImage>,
    ) -> Result<Token, Self::Error>;
}
