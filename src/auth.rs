use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequest {
    pub client_id: String,
    pub client_secret: String,
    pub non_expiring: Option<bool>,
}
