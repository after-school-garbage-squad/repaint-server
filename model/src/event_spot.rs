use crate::id::Id;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventSpot {
    pub spot_id: Id<Self>,
    pub name: String,
    pub is_pick: bool,
}
