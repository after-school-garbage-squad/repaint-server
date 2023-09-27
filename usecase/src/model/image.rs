use repaint_server_model::event_image::Image as EventImage;
use repaint_server_model::id::Id;
use repaint_server_model::visitor::Visitor;
use repaint_server_model::visitor_image::Image as VisitorImage;
use repaint_server_model::{event::Event, visitor_image::CurrentImage};
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCurrentRequest {
    pub event_id: Id<Event>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRequest {
    pub event_id: Id<Event>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyRequest {
    pub event_id: Id<Event>,
    pub image_id: Id<CurrentImage>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCurrentRequest {
    pub event_id: Id<Event>,
    pub image_id: Id<VisitorImage>,
}
