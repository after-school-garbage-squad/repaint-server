use repaint_server_model::event::{Contact, Event as EventModel};
use repaint_server_model::event_beacon::EventBeacon;
use repaint_server_model::event_image::Image;
use repaint_server_model::id::Id;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub event_id: Id<EventModel>,
    pub name: String,
    pub hp_url: String,
    pub contact: Contact,
    pub beacons: Vec<EventBeacon>,
    pub images: Vec<Id<Image>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEvent {
    pub name: String,
    pub hp_url: String,
    pub contact: Contact,
    pub images: Vec<Id<Image>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEvent {
    pub name: String,
    pub hp_url: String,
    pub contact: Contact,
    pub images: Vec<Id<Image>>,
}
