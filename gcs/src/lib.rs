#![warn(unreachable_pub)]

use async_trait::async_trait;
use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::http::objects::upload::{Media, UploadObjectRequest, UploadType};
use google_cloud_storage::http::Error;
use repaint_server_model::event::Event;
use repaint_server_model::event_image::Image as EventImage;
use repaint_server_model::id::Id;
use repaint_server_model::visitor_image::Image as VisitorImage;
use repaint_server_usecase::infra::gcs::GoogleCloudStorage;
use teloc::dev::DependencyClone;
use tracing::info;

#[derive(Clone)]
pub struct Gcs {
    client: Client,
    bucket: String,
}

impl Gcs {
    pub async fn new(bucket: String) -> Self {
        let config = ClientConfig::default()
            .with_auth()
            .await
            .expect("failed to create config");
        let client = Client::new(config);
        info!("initialized GCS client");

        Self { client, bucket }
    }
}

impl DependencyClone for Gcs {}

#[async_trait]
impl GoogleCloudStorage for Gcs {
    type Error = Error;

    async fn upload_event_image(
        &self,
        data: Vec<u8>,
        event_id: Id<Event>,
        image_id: Id<EventImage>,
    ) -> Result<(), Self::Error> {
        let upload_type = UploadType::Simple(Media::new(format!(
            "{}/image/{}_origin.png",
            event_id, image_id
        )));
        match self
            .client
            .upload_object(
                &UploadObjectRequest {
                    bucket: self.bucket.clone(),
                    ..Default::default()
                },
                data,
                &upload_type,
            )
            .await
        {
            Ok(_) => info!("uploaded event image"),
            Err(e) => return Err(e),
        }

        Ok(())
    }

    async fn upload_visitor_image(
        &self,
        data: Vec<u8>,
        event_id: Id<Event>,
        image_id: Id<VisitorImage>,
    ) -> Result<(), Self::Error> {
        let upload_type = UploadType::Simple(Media::new(format!(
            "{}/image/{}_origin.png",
            event_id, image_id
        )));
        match self
            .client
            .upload_object(
                &UploadObjectRequest {
                    bucket: self.bucket.clone(),
                    ..Default::default()
                },
                data,
                &upload_type,
            )
            .await
        {
            Ok(_) => info!("uploaded visitor image"),
            Err(e) => return Err(e),
        }

        Ok(())
    }
}
