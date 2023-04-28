use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    pub id: String,
    pub url: String,
    pub event: WebhookEvent,
    pub created_at: String,          // date
    pub updated_at: String,          // date
    pub disabled_at: Option<String>, // date
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWebhookRequest<'a> {
    pub event: WebhookEvent,
    pub url: String,
    pub headers: Option<&'a HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWebhookRequest<'a> {
    pub event: Option<WebhookEvent>,
    pub url: Option<String>,
    pub headers: Option<&'a HashMap<String, String>>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WebhookEvent {
    #[serde(rename = "item/created")]
    ItemCreated,
    #[serde(rename = "item/updated")]
    ItemUpdated,
    #[serde(rename = "item/error")]
    ItemError,
    #[serde(rename = "item/deleted")]
    ItemDeleted,
    #[serde(rename = "item/waiting_user_input")]
    ItemWaitingUserInput,
    #[serde(rename = "item/login_succeeded")]
    ItemLoginSucceeded,
    #[serde(rename = "connector/status_updated")]
    ConnectorStatusUpdated,
    #[serde(rename = "transactions/deleted")]
    TransactionsDeleted,
    #[serde(rename = "all")]
    All,
}
