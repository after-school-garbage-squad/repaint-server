use async_trait::async_trait;
use repaint_server_model::event::Event;
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_model::visitor_image::CurrentImage;
use repaint_server_model::{AsyncSafe, StaticError};

use crate::model::otp::Token;

#[async_trait]
pub trait ImageOtp: AsyncSafe {
    type Error: StaticError;

    async fn verify(
        &self,
        event_id: Id<Event>,
        image_id: Id<CurrentImage>,
        visitor_id: Id<Visitor>,
    ) -> Result<Token, Self::Error>;
}
