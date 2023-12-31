use serde::Serialize;

use crate::id::Id;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventImage {
    pub image_id: Id<Image>,
}

#[derive(Debug)]
pub struct Image {}
