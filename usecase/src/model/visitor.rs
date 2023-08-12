use repaint_server_model::event::Event;
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor as VisitorModel;
use repaint_server_model::visitor_image::{CurrentImage, Image};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Visitor {
    pub visitor_identification: VisitorIdentification,
    pub registration_id: String,
    pub palettes: Vec<i32>,
    pub image_id: Id<Image>,
    pub current_image_id: Id<CurrentImage>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisitorIdentification {
    pub event_id: Id<Event>,
    pub visitor_id: Id<VisitorModel>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterVisitor {
    pub event_id: Id<Event>,
    pub registration_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeVisitor {
    pub visitor_identification: VisitorIdentification,
    pub registration_id: String,
}
