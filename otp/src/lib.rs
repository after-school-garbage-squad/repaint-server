#![warn(missing_debug_implementations)]
#![warn(unreachable_pub)]

use async_trait::async_trait;
use repaint_server_model::event::Event;
use repaint_server_model::event_image::Image as EventImage;
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_model::visitor_image::CurrentImage;
use repaint_server_usecase::infra::otp::ImageOtp;
use repaint_server_usecase::model::otp::Token;
use reqwest::{Client, Error};
use teloc::dev::DependencyClone;
use tracing::info;

#[derive(Debug, Clone)]
pub struct Otp {
    client: Client,
    origin: String,
    url: String,
    bucket: String,
}

impl Otp {
    pub fn new(origin: String, url: String, bucket: String) -> Self {
        info!("initialized OTP client");

        Self {
            client: Client::new(),
            origin,
            url,
            bucket,
        }
    }
}

impl DependencyClone for Otp {}

#[async_trait]
impl ImageOtp for Otp {
    type Error = Error;

    async fn verify_current(
        &self,
        event_id: Id<Event>,
        image_id: Id<CurrentImage>,
        visitor_id: Id<Visitor>,
    ) -> Result<Token, Self::Error> {
        let res = self
            .client
            .post(format!(
                "{}/token?url={}/{}/{}/image/{}_current_{}.png",
                self.origin, self.url, self.bucket, event_id, image_id, visitor_id
            ))
            .send()
            .await?
            .json::<Token>()
            .await?;
        info!("verified current image: {:?}", res);

        Ok(res)
    }

    async fn verify_event(
        &self,
        event_id: Id<Event>,
        image_id: Id<EventImage>,
    ) -> Result<Token, Self::Error> {
        let res = self
            .client
            .post(format!(
                "{}/token?url={}/{}/{}/image/{}_origin.png",
                self.origin, self.url, self.bucket, event_id, image_id
            ))
            .send()
            .await?
            .json::<Token>()
            .await?;
        info!("verified event image: {:?}", res);

        Ok(res)
    }
}
