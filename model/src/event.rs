use email_address::EmailAddress;
use serde::{Deserialize, Serialize};

use crate::id::Id;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub event_id: Id<Self>,
    pub name: String,
    pub hp_url: String,
    pub contact: Contact,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Contact {
    pub name: String,
    pub email: EmailAddress,
    pub phone: String,
}
