use serde::Serialize;

use crate::id::Id;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Admin {
    pub admin_id: Id<Self>,
    pub subject: String,
}
