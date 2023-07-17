use crate::id::Id;

#[derive(Debug, Clone, PartialEq, Eq)]
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
