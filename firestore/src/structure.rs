use repaint_server_model::event_spot::EventSpot;
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PaletteStructure {
    pub(crate) palette_id: Option<i32>,
    pub(crate) palettes_ids: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct VisitorStructure {
    pub(crate) spot_id: Id<EventSpot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct AdminStructure {
    pub(crate) event_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct VisitorLogStructure {
    pub(crate) visitor_id: Id<Visitor>,
    pub(crate) spot_id: Id<EventSpot>,
    pub(crate) palettes_length: usize,
    pub(crate) took_photo: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SpotLogStructure {
    pub(crate) spot_id: Id<EventSpot>,
    pub(crate) head_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TrafficLogStructure {
    pub(crate) collection: String,
    pub(crate) document: String,
    pub(crate) from: Id<EventSpot>,
    pub(crate) to: Id<EventSpot>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RegisterLogStructure {
    pub(crate) collection: String,
    pub(crate) document: String,
    pub(crate) visitor_id: Id<Visitor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct InitializeLogStructure {
    pub(crate) collection: String,
    pub(crate) document: String,
    pub(crate) visitor_id: Id<Visitor>,
}
