use repaint_server_model::event_image::Image as EventImage;
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckVisitorRequest {
    pub visitor_id: Id<Visitor>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDefaultRequest {
    pub image_id: Id<EventImage>,
}
