use repaint_server_model::event::Event;
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor as VisitorModel;
use repaint_server_model::visitor_image::{CurrentImage, Image};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisitorResponse {
    pub visitor_identification: VisitorIdentification,
    pub registration_id: String,
    pub palettes: Vec<i32>,
    pub image_id: Option<Id<Image>>,
    pub current_image_id: Id<CurrentImage>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisitorIdentification {
    pub event_id: Id<Event>,
    pub visitor_id: Id<VisitorModel>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterVisitorResponse {
    pub visitor_identification: VisitorIdentification,
    pub registration_id: String,
    pub palettes: Vec<i32>,
}
