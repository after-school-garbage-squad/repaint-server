use repaint_server_model::event_spot::{EventSpot, IBeacon};
use repaint_server_model::id::Id;
use serde::Serialize;

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
    pub i_beacon: IBeacon,
    pub hw_id: String,
    pub service_uuid: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrafficStatus {
    pub spot_id: Id<EventSpot>,
    pub head_count: usize,
}
