#![warn(missing_debug_implementations)]
#![warn(unreachable_pub)]

use async_trait::async_trait;
use repaint_server_usecase::infra::fcm::FirebaseCloudMessaging;
use reqwest::{Client, Error};
use serde_json::json;
use teloc::dev::DependencyClone;

#[derive(Debug, Clone)]
pub struct Fcm {
    client: Client,
    project_id: String,
}

impl Fcm {
    pub fn new(project_id: &str) -> Self {
        Self {
            client: Client::new(),
            project_id: project_id.to_string(),
        }
    }
}

impl DependencyClone for Fcm {}

#[async_trait]
impl FirebaseCloudMessaging for Fcm {
    type Error = Error;

    async fn send(&self, registeration_id: String, spot_name: String) -> Result<(), Self::Error> {
        self.client
            .post(format!(
                "https://fcm.googleapis.com/v1/projects/{}/messages:send",
                self.project_id
            ))
            .json(&json!(
                {
                    "message": {
                        "token": registeration_id,
                        "notification": {
                            "title": "Re:paint",
                            "body": format!(include_str!("./message.tmp.txt"), SPOT_NAME = spot_name)
                        }
                    }
                }
            ))
            .send()
            .await?;

        Ok(())
    }
}
