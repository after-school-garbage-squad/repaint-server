use repaint_server_model::event_spot::EventSpot;
use repaint_server_model::id::Id;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrafficStatus {
    pub spot_id: Id<EventSpot>,
    pub head_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnableBonusRequest {
    pub from: Id<EventSpot>,
    pub to: Id<EventSpot>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTrafficStatusResponse {
    pub traffics: Vec<TrafficStatus>,
}
