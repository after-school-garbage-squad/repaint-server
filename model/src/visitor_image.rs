use crate::id::Id;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisitorImage {
    pub image_id: Id<Image>,
    pub compressed_image_id: Id<CompressedImage>,
    pub current_image_id: Id<CurrentImage>,
}

#[derive(Debug)]
pub struct Image {}

#[derive(Debug)]
pub struct CompressedImage {}

#[derive(Debug)]
pub struct CurrentImage {}
