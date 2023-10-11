use repaint_server_model::event::Event;
use repaint_server_model::event_spot::EventSpot;
use repaint_server_model::id::Id;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotResponse {
    pub spot_id: Id<EventSpot>,
    pub name: String,
    pub beacon: Beacon,
    pub is_pick: bool,
    pub bonus: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Beacon {
    pub hw_id: String,
    pub service_uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRequest {
    pub spot_id: Id<EventSpot>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub name: String,
    pub hw_id: String,
    pub service_uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    pub spot_id: Id<EventSpot>,
    pub name: String,
    pub is_pick: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScannedRequest {
    pub event_id: Id<Event>,
    pub hw_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckByBeaconQuery {
    pub hw_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckByQrQuery {
    pub spot_id: Id<EventSpot>,
}
