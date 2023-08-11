use crate::id::Id;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventSpot {
    pub spot_id: Id<Self>,
    pub name: String,
    pub is_pick: bool,
}
