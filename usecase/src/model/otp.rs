use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub full: String,
    pub id: Option<i32>,
    pub token: String,
    pub url: String,
    pub limit_times: i32,
    pub expires_at: String,
}
