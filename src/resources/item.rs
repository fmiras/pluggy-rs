use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{Connector, ConnectorCredential, ExecutionErrorResult, ExecutionStatus};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemStatus {
    Updated,
    Updating,
    WaitingUserInput,
    LoginError,
    Outdated,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemProductsStatusDetail {
    pub accounts: ItemProductState,
    pub credit_cards: ItemProductState,
    pub transactions: ItemProductState,
    pub investments: ItemProductState,
    pub identity: ItemProductState,
    pub payment_data: ItemProductState,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemProductState {
    pub is_updated: bool,
    pub last_updated_at: Option<String>, // date
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAction {
    pub instructions: String,
    pub attributes: Option<HashMap<String, String>>,
    pub expires_at: Option<String>, // date
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub connector: Connector,
    pub status: ItemStatus,
    pub status_detail: Option<ItemProductsStatusDetail>,
    pub error: Option<ExecutionErrorResult>,
    pub execution_status: ExecutionStatus,
    pub created_at: String,              // date
    pub updated_at: String,              // date
    pub last_updated_at: Option<String>, // date
    pub parameter: Option<ConnectorCredential>,
    pub webhook_url: Option<String>,
    pub client_user_id: Option<String>,
    pub user_action: Option<UserAction>,
    pub consecutive_failed_login_attempts: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateItemRequest<'a> {
    // TODO support additional fields
    // pub client_user_id: Option<String>,
    // pub webhook_url: Option<String>,
    pub connector_id: i32,
    pub parameters: &'a HashMap<String, String>,
}

#[derive(Serialize)]
pub struct UpdateItemRequest<'a> {
    pub parameters: &'a HashMap<String, String>,
}
