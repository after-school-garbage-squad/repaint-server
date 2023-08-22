use repaint_server_model::event::{Contact, Event as EventModel};
use repaint_server_model::event_image::Image;
use repaint_server_model::event_spot::EventSpot;
use repaint_server_model::id::Id;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventResponse {
    pub event_id: Id<EventModel>,
    pub name: String,
    pub hp_url: String,
    pub contact: Contact,
    pub spots: Vec<EventSpot>,
    pub images: Vec<Id<Image>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventResponse {
    pub event_id: Id<EventModel>,
    pub name: String,
    pub hp_url: String,
    pub contact: Contact,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEventResponse {
    pub event_id: Id<EventModel>,
    pub name: String,
    pub hp_url: String,
    pub contact: Contact,
}
