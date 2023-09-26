#![warn(missing_debug_implementations)]
#![warn(unreachable_pub)]

use async_trait::async_trait;
use google_cloud_auth::token::DefaultTokenSourceProvider;
use google_cloud_auth::{credentials::CredentialsFile, project::Config};
use google_cloud_token::TokenSourceProvider;
use repaint_server_usecase::infra::fcm::FirebaseCloudMessaging;
use reqwest::{header::HeaderMap, Client, ClientBuilder, Error};
use serde_json::json;
use teloc::dev::DependencyClone;

#[derive(Debug, Clone)]
pub struct Fcm {
    client: Client,
    project_id: String,
}

impl Fcm {
    /// Please set `GOOGLE_APPLICATION_CREDENTIALS_JSON` environment variable.
    pub async fn new(project_id: &str) -> Self {
        let scopes = ["https://www.googleapis.com/auth/firebase.messaging"];
        let config = Config {
            audience: None,
            scopes: Some(&scopes),
            sub: None,
        };
        let cred = CredentialsFile::new()
            .await
            .expect("failed to create credentials file");
        let ts = DefaultTokenSourceProvider::new_with_credentials(config, Box::new(cred))
            .await
            .expect("failed to create token source");
        let bearer_token = ts
            .token_source()
            .token()
            .await
            .expect("failed to get bearer token");
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            bearer_token.parse().expect("failed to parse bearer token"),
        );

        Self {
            client: ClientBuilder::new()
                .default_headers(headers)
                .build()
                .expect("failed to create client"),
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
