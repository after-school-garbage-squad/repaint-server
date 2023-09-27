#![warn(missing_debug_implementations)]
#![warn(unreachable_pub)]

use async_trait::async_trait;
use google_cloud_gax::grpc::Status;
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::client::{Client, ClientConfig};
use repaint_server_model::event::Event;
use repaint_server_model::event_image::Image as EventImage;
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_model::visitor_image::Image as VisitorImage;
use repaint_server_usecase::infra::pubsub::GoogleCloudPubSub;
use teloc::dev::DependencyClone;

#[derive(Debug, Clone)]
pub struct PubSub {
    client: Client,
    cluster: i32,
    clustering_topic: String,
    merge_topic: String,
}

impl PubSub {
    /// Please set `GOOGLE_APPLICATION_CREDENTIALS_JSON` environment variable.
    pub async fn new(cluster: i32, clustering_topic: String, merge_topic: String) -> Self {
        let config = ClientConfig::default()
            .with_auth()
            .await
            .expect("failed to create config");
        let client = Client::new(config).await.expect("failed to create client");

        Self {
            client,
            cluster,
            clustering_topic,
            merge_topic,
        }
    }
}

impl DependencyClone for PubSub {}

#[async_trait]
impl GoogleCloudPubSub for PubSub {
    type Error = Status;

    async fn publish_clustering_event_image(
        &self,
        event_id: Id<Event>,
        image_id: Id<EventImage>,
    ) -> Result<(), Self::Error> {
        let topic = self.client.topic(&self.clustering_topic);
        if !topic.exists(None).await? {
            topic.create(None, None).await?;
        }
        let mut publisher = topic.new_publisher(None);
        let msg = PubsubMessage {
            data: serde_json::json!({
                "event_id": event_id,
                "visitor_id": None::<Id<Visitor>>,
                "image_id": image_id,
                "palette_max": self.cluster
            })
            .to_string()
            .into(),
            ..Default::default()
        };
        let awaiter = publisher.publish(msg).await;
        let _ = awaiter.get().await?;
        publisher.shutdown().await;

        Ok(())
    }

    async fn publish_clustering_visitor_image(
        &self,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
        image_id: Id<VisitorImage>,
    ) -> Result<(), Self::Error> {
        let topic = self.client.topic(&self.clustering_topic);
        if !topic.exists(None).await? {
            topic.create(None, None).await?;
        }
        let mut publisher = topic.new_publisher(None);
        let msg = PubsubMessage {
            data: serde_json::json!({
                "event_id": event_id,
                "visitor_id": visitor_id,
                "image_id": image_id,
                "palette_max": self.cluster
            })
            .to_string()
            .into(),
            ..Default::default()
        };
        let awaiter = publisher.publish(msg).await;
        let _ = awaiter.get().await?;
        publisher.shutdown().await;

        Ok(())
    }

    async fn publish_merge_current_image(
        &self,
        event_id: Id<Event>,
        visitor_id: Id<Visitor>,
        image_id: Id<VisitorImage>,
        palette_ids: Vec<i32>,
    ) -> Result<(), Self::Error> {
        let topic = self.client.topic(&self.merge_topic);
        if !topic.exists(None).await? {
            topic.create(None, None).await?;
        }
        let mut publisher = topic.new_publisher(None);
        let msg = PubsubMessage {
            data: serde_json::json!({
                "event_id": event_id,
                "visitor_id": visitor_id,
                "image_id": image_id,
                "palette_ids": palette_ids
            })
            .to_string()
            .into(),
            ..Default::default()
        };
        let awaiter = publisher.publish(msg).await;
        let _ = awaiter.get().await?;
        publisher.shutdown().await;

        Ok(())
    }
}
