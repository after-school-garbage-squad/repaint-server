use repaint_server_model::event_beacon::EventBeacon;
use repaint_server_model::event_spot::EventSpot;
use repaint_server_model::id::Id;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Spot {
    pub spot_id: Id<EventSpot>,
    pub name: String,
    pub beacon: EventBeacon,
    pub pick_qr: Option<Id<EventSpot>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterSpot {
    pub name: String,
    pub beacon: EventBeacon,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSpot {
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
