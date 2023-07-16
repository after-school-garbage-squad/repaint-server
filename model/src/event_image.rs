use crate::id::Id;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventImage {
    pub image_id: Id<Image>,
    pub compressed_image_id: Id<CompressedImage>,
}

#[derive(Debug)]
pub struct Image {}

#[derive(Debug)]
pub struct CompressedImage {}
