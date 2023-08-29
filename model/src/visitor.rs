use serde::Serialize;

use crate::id::Id;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Visitor {
    pub id: i32,
    pub visitor_id: Id<Self>,
    pub registration_id: String,
}
