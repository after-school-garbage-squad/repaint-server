#![warn(missing_debug_implementations)]
#![warn(unreachable_pub)]

use async_trait::async_trait;
use repaint_server_model::event::Event;
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_model::visitor_image::CurrentImage;
use repaint_server_usecase::{infra::otp::ImageOtp, model::otp::Token};
use reqwest::{Client, Error};
use teloc::dev::DependencyClone;

#[derive(Debug, Clone)]
pub struct Otp {
    client: Client,
    gcs_url: String,
    origin: String,
}

impl Otp {
    pub fn new(bucket: String, origin: String) -> Self {
        Self {
            client: Client::new(),
            gcs_url: format!("gs://{}", bucket),
            origin,
        }
    }
}

impl DependencyClone for Otp {}

#[async_trait]
impl ImageOtp for Otp {
    type Error = Error;

    async fn verify(
        &self,
        event_id: Id<Event>,
        image_id: Id<CurrentImage>,
        visitor_id: Id<Visitor>,
    ) -> Result<Token, Self::Error> {
        let res = self
            .client
            .post(format!(
                "{}/token?url={}/{}/image/${}_current_{}.png",
                self.origin, self.gcs_url, event_id, visitor_id, image_id
            ))
            .send()
            .await?
            .json::<Token>()
            .await?;

        Ok(res)
    }
}
