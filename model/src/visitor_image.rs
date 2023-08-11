use crate::id::Id;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisitorImage {
    pub image_id: Id<Image>,
    pub current_image_id: Id<CurrentImage>,
}

#[derive(Debug)]
pub struct Image {}

#[derive(Debug)]
pub struct CurrentImage {}
