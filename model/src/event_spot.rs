use serde::Serialize;

use crate::id::Id;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventSpot {
    pub spot_id: Id<Self>,
    pub name: String,
    pub is_pick: bool,
    pub bonus: bool,
    pub i_beacon: IBeacon,
    pub hw_id: String,
    pub service_uuid: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IBeacon {
    pub major: i16,
    pub minor: i16,
    pub beacon_uuid: String,
}
