use repaint_server_model::event_spot::EventSpot;
use repaint_server_model::id::Id;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrafficStatus {
    pub spot_id: Id<EventSpot>,
    pub head_count: usize,
}
