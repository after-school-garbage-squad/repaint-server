use serde::Serialize;

use crate::id::Id;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisitorImage {
    pub image_id: Option<Id<Image>>,
    pub current_image_id: Id<CurrentImage>,
}

#[derive(Debug)]
pub struct Image {}

#[derive(Debug)]
pub struct CurrentImage {}
