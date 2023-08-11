use crate::id::Id;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Visitor {
    pub visitor_id: Id<Self>,
    pub registration_id: String,
}
