use crate::id::Id;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Visitor {
    pub visitor_id: Id<Self>,
    pub registration_id: String,
}
