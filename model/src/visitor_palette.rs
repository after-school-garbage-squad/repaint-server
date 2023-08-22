use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisitorPalette {
    pub palette_id_list: Vec<i32>,
}
