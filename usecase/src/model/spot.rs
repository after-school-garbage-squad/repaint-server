use repaint_server_model::event_beacon::EventBeacon;
use repaint_server_model::event_spot::EventSpot;
use repaint_server_model::id::Id;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotResponse {
    pub spot_id: Id<EventSpot>,
    pub name: String,
    pub beacon: EventBeacon,
    pub pick_qr: Option<Id<EventSpot>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterSpotRequest {
    pub name: String,
    pub beacon: EventBeacon,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSpotRequest {
    pub name: String,
    pub beacon: EventBeacon,
    pub is_pick: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrafficStatus {
    pub spot_id: Id<EventSpot>,
    pub head_count: u32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllTrafficRequest {
    pub from: Id<EventSpot>,
    pub to: Id<EventSpot>,
}
