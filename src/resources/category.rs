use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: String,
    pub description: String,
    pub parent_id: Option<String>,
    pub parent_description: Option<String>,
}
