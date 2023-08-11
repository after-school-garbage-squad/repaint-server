use crate::id::Id;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventImage {
    pub image_id: Id<Image>,
    pub compressed_image_id: Id<CompressedImage>,
}

#[derive(Debug)]
pub struct Image {}

#[derive(Debug)]
pub struct CompressedImage {}
