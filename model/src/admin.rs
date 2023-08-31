use serde::Serialize;

use crate::id::Id;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Admin {
    pub id: i32,
    pub admin_id: Id<Self>,
    pub subject: String,
}
