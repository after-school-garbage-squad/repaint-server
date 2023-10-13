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
use tokio::task::JoinHandle;
use tracing::info;

#[derive(Debug, Clone)]
pub struct PubSub {
    client: Client,
    cluster: i32,
    clustering_topic: String,
    merge_topic: String,
    notification_topic: String,
}

impl PubSub {
    pub async fn new(
        cluster: i32,
        clustering_topic: String,
        merge_topic: String,
        notification_topic: String,
    ) -> Self {
        let config = ClientConfig::default()
            .with_auth()
            .await
            .expect("failed to create config");
        let client = Client::new(config).await.expect("failed to create client");
        info!("initialized PubSub client");

        Self {
            client,
            cluster,
            clustering_topic,
            merge_topic,
            notification_topic,
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
        let publisher = topic.new_publisher(None);
        let tasks = (0..1)
            .into_iter()
            .map(|_i| {
                let publisher = publisher.clone();
                let cluster = self.cluster;
                tokio::spawn(async move {
                    let msg = PubsubMessage {
                        data: serde_json::json!({
                            "event_id": event_id,
                            "visitor_id": None::<Id<Visitor>>,
                            "image_id": image_id,
                            "palette_max": cluster
                        })
                        .to_string()
                        .into(),
                        ..Default::default()
                    };
                    let awaiter = publisher.publish(msg).await;

                    awaiter.get().await
                })
            })
            .collect::<Vec<JoinHandle<Result<_, _>>>>();
        for task in tasks {
            let message_id = task.await.expect("failed to join")?;
            info!("published clustering event image: {}", message_id);
        }
        let mut publisher = publisher;
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
        let publisher = topic.new_publisher(None);
        let tasks = (0..1)
            .into_iter()
            .map(|_i| {
                let publisher = publisher.clone();
                let cluster = self.cluster;
                tokio::spawn(async move {
                    let msg = PubsubMessage {
                        data: serde_json::json!({
                            "event_id": event_id,
                            "visitor_id": visitor_id,
                            "image_id": image_id,
                            "palette_max": cluster
                        })
                        .to_string()
                        .into(),
                        ..Default::default()
                    };
                    let awaiter = publisher.publish(msg).await;

                    awaiter.get().await
                })
            })
            .collect::<Vec<JoinHandle<Result<_, _>>>>();
        for task in tasks {
            let message_id = task.await.expect("failed to join")?;
            info!("published clustering visitor image: {}", message_id);
        }
        let mut publisher = publisher;
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
        let publisher = topic.new_publisher(None);
        let tasks = (0..1)
            .into_iter()
            .map(|_i| {
                let publisher = publisher.clone();
                let palette_ids = palette_ids.clone();
                tokio::spawn(async move {
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

                    awaiter.get().await
                })
            })
            .collect::<Vec<JoinHandle<Result<_, _>>>>();
        for task in tasks {
            let message_id = task.await.expect("failed to join")?;
            info!("published merge current image: {}", message_id);
        }
        let mut publisher = publisher;
        publisher.shutdown().await;

        Ok(())
    }

    async fn publish_notification(
        &self,
        registration_id: String,
        spot_name: String,
    ) -> Result<(), Self::Error> {
        let topic = self.client.topic(&self.notification_topic);
        if !topic.exists(None).await? {
            topic.create(None, None).await?;
        }
        let publisher = topic.new_publisher(None);
        let tasks = (0..1)
            .into_iter()
            .map(|_i| {
                let publisher = publisher.clone();
                let registration_id = registration_id.clone();
                let body = format!(include_str!("./message.tmp.txt"), SPOT_NAME = spot_name);
                tokio::spawn(async move {
                    let msg = PubsubMessage {
                        data: serde_json::json!({
                            "title": "Re:paintからのお知らせ",
                            "body": body,
                            "registration_id": registration_id
                        })
                        .to_string()
                        .into(),
                        ..Default::default()
                    };
                    let awaiter = publisher.publish(msg).await;

                    awaiter.get().await
                })
            })
            .collect::<Vec<JoinHandle<Result<_, _>>>>();
        for task in tasks {
            let message_id = task.await.expect("failed to join")?;
            info!("published notification: {}", message_id);
        }
        let mut publisher = publisher;
        publisher.shutdown().await;

        Ok(())
    }

    async fn publish_pick_notification(
        &self,
        registration_id: String,
        spot_name: String,
    ) -> Result<(), Self::Error> {
        let topic = self.client.topic(&self.notification_topic);
        if !topic.exists(None).await? {
            topic.create(None, None).await?;
        }
        let publisher = topic.new_publisher(None);
        let tasks = (0..1)
            .into_iter()
            .map(|_i| {
                let publisher = publisher.clone();
                let registration_id = registration_id.clone();
                let title = format!("{}の近くにいます", spot_name);
                tokio::spawn(async move {
                    let msg = PubsubMessage {
                        data: serde_json::json!({
                            "title": title,
                            "body": "アプリからQRコードをスキャンして、パレットを入手しましょう!",
                            "registration_id": registration_id
                        })
                        .to_string()
                        .into(),
                        ..Default::default()
                    };
                    let awaiter = publisher.publish(msg).await;

                    awaiter.get().await
                })
            })
            .collect::<Vec<JoinHandle<_>>>();
        for task in tasks {
            let message_id = task.await.expect("failed to join")?;
            info!("published notification: {}", message_id);
        }
        let mut publisher = publisher;
        publisher.shutdown().await;

        Ok(())
    }
}
