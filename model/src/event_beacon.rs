use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventBeacon {
    pub i_beacon: IBeacon,
    pub hw_id: Option<String>,
    pub service_uuid: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IBeacon {
    pub major: i16,
    pub minor: i16,
    pub beacon_uuid: String,
}
