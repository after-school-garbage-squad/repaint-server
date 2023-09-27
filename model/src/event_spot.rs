use serde::Serialize;

use crate::id::Id;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventSpot {
    pub spot_id: Id<Self>,
    pub name: String,
    pub is_pick: bool,
    pub bonus: bool,
    pub hw_id: String,
    pub service_uuid: String,
}
