use std::collections::HashMap;

use hyper::http::request::Builder;
use hyper::{Body, Client as HyperClient, Method, Request, StatusCode};
use hyper_tls::HttpsConnector;
use url::Url;

use crate::auth::*;
use crate::resources::*;

mod auth;
mod resources;

pub struct Client {
    client_id: String,
    client_secret: String,
    url: String,
    client: HyperClient<HttpsConnector<hyper::client::connect::HttpConnector>>,
}

pub fn authenticated_request_builder(method: Method, url: &Url, api_key: &str) -> Builder {
    Request::builder()
        .method(method)
        .uri(url.as_str())
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("X-API-KEY", api_key)
}

impl Client {
    pub fn new(client_id: String, client_secret: String) -> Self {
        let client = HyperClient::builder().build::<_, hyper::Body>(HttpsConnector::new());

        Self {
            client_id,
            client_secret,
            url: "https://api.pluggy.ai".to_string(),
            client,
        }
    }

    pub fn new_from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let client_id = dotenv::var("PLUGGY_CLIENT_ID")?;
        let client_secret = dotenv::var("PLUGGY_CLIENT_SECRET")?;
        let client = Self::new(client_id, client_secret);
        Ok(client)
    }

    pub async fn new_from_env_with_api_key() -> Result<(Self, String), Box<dyn std::error::Error>> {
        let client = Self::new_from_env()?;
        let connect_token = client.create_api_key().await?;
        Ok((client, connect_token))
    }

    async fn create_api_key(&self) -> Result<String, Box<dyn std::error::Error>> {
        let url = Url::parse(&format!("{}/auth", self.url))?;

        let payload = AuthRequest {
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            non_expiring: None,
        };

        let json_payload = serde_json::to_string(&payload)?;

        let request = Request::builder()
            .method(Method::POST)
            .uri(url.as_str())
            .header("Content-Type", "application/json")
            .body(Body::from(json_payload))?;

        let response = self.client.request(request).await?;
        let body = hyper::body::to_bytes(response.into_body()).await?;
        let body = String::from_utf8(body.to_vec())?;
        let json: serde_json::Value = serde_json::from_str(&body)?;
        let api_key = json["apiKey"].as_str();

        match api_key {
            Some(api_key) => Ok(api_key.to_string()),
            None => Err("No api key found".into()),
        }
    }

    pub async fn create_connect_token(
        &self,
        api_key: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = Url::parse(&format!("{}/connect_token", self.url))?;

        let request =
            authenticated_request_builder(Method::POST, &url, api_key).body(Body::empty())?;
        let response = self.client.request(request).await?;

        let body: hyper::body::Bytes = hyper::body::to_bytes(response.into_body()).await?;
        let body = String::from_utf8(body.to_vec())?;
        let json: serde_json::Value = serde_json::from_str(&body)?;
        let connect_token = json["accessToken"].as_str();

        match connect_token {
            Some(connect_token) => Ok(connect_token.to_string()),
            None => Err("No connect token found".into()),
        }
    }

    pub async fn get_connectors(
        &self,
        api_key: &str,
        with_sandbox: bool,
    ) -> Result<Vec<Connector>, Box<dyn std::error::Error>> {
        let mut url = Url::parse(&format!("{}/connectors", self.url))?;
        url.query_pairs_mut()
            .append_pair("sandbox", &with_sandbox.to_string());

        let request =
            authenticated_request_builder(Method::GET, &url, api_key).body(Body::empty())?;
        let response = self.client.request(request).await?;

        let body: hyper::body::Bytes = hyper::body::to_bytes(response.into_body()).await?;
        let json: PageResponse<Connector> = serde_json::from_slice(&body)?;

        Ok(json.results)
    }

    pub async fn get_connector(
        &self,
        api_key: &str,
        connector_id: &str,
    ) -> Result<Connector, Box<dyn std::error::Error>> {
        let url: Url = Url::parse(&format!("{}/connectors/{}", self.url, connector_id))?;

        let request =
            authenticated_request_builder(Method::GET, &url, api_key).body(Body::empty())?;
        let response = self.client.request(request).await?;

        let body: hyper::body::Bytes = hyper::body::to_bytes(response.into_body()).await?;
        let json: Connector = serde_json::from_slice(&body)?;

        Ok(json)
    }

    pub async fn get_item(
        &self,
        api_key: &str,
        item_id: &str,
    ) -> Result<Item, Box<dyn std::error::Error>> {
        let url: Url = Url::parse(&format!("{}/items/{}", self.url, item_id))?;

        let request =
            authenticated_request_builder(Method::GET, &url, api_key).body(Body::empty())?;
        let response = self.client.request(request).await?;

        let body: hyper::body::Bytes = hyper::body::to_bytes(response.into_body()).await?;
        let json: Item = serde_json::from_slice(&body)?;

        Ok(json)
    }

    pub async fn validate_parameters(
        &self,
        api_key: &str,
        connector_id: i32,
        parameters: &HashMap<&str, &str>,
    ) -> Result<ValidationResult, Box<dyn std::error::Error>> {
        let url = Url::parse(&format!(
            "{}/connectors/{}/validate",
            self.url, connector_id
        ))?;
        let request = authenticated_request_builder(Method::POST, &url, api_key)
            .body(Body::from(serde_json::to_string(parameters)?))?;
        let response = self.client.request(request).await?;

        let body: hyper::body::Bytes = hyper::body::to_bytes(response.into_body()).await?;
        let json: ValidationResult = serde_json::from_slice(&body)?;

        Ok(json)
    }

    pub async fn create_item(
        &self,
        api_key: &str,
        connector_id: i32,
        parameters: &HashMap<String, String>,
    ) -> Result<Item, Box<dyn std::error::Error>> {
        let url = Url::parse(&format!("{}/items", self.url))?;

        let create_item_request = CreateItemRequest {
            connector_id,
            parameters,
        };

        let request = authenticated_request_builder(Method::POST, &url, api_key)
            .body(Body::from(serde_json::to_string(&create_item_request)?))?;
        let response = self.client.request(request).await?;

        let body: hyper::body::Bytes = hyper::body::to_bytes(response.into_body()).await?;
        let json: Item = serde_json::from_slice(&body)?;

        Ok(json)
    }

    pub async fn update_item(
        &self,
        api_key: &str,
        item_id: &str,
        parameters: &HashMap<String, String>,
    ) -> Result<Item, Box<dyn std::error::Error>> {
        let url = Url::parse(&format!("{}/items/{}", self.url, item_id))?;

        let update_item_request = UpdateItemRequest { parameters };

        let request = authenticated_request_builder(Method::PATCH, &url, api_key)
            .body(Body::from(serde_json::to_string(&update_item_request)?))?;
        let response = self.client.request(request).await?;

        let body: hyper::body::Bytes = hyper::body::to_bytes(response.into_body()).await?;
        let json: Item = serde_json::from_slice(&body)?;

        Ok(json)
    }

    pub async fn update_item_mfa_credentials(
        &self,
        api_key: &str,
        item_id: &str,
        parameters: &HashMap<String, String>,
    ) -> Result<Item, Box<dyn std::error::Error>> {
        let url = Url::parse(&format!("{}/items/{}/mfa", self.url, item_id))?;

        let update_item_mfa_credentials_request = UpdateItemRequest { parameters };

        let request = authenticated_request_builder(Method::PATCH, &url, api_key).body(
            Body::from(serde_json::to_string(&update_item_mfa_credentials_request)?),
        )?;
        let response = self.client.request(request).await?;

        let body: hyper::body::Bytes = hyper::body::to_bytes(response.into_body()).await?;
        let json: Item = serde_json::from_slice(&body)?;

        Ok(json)
    }

    pub async fn delete_item(
        &self,
        api_key: &str,
        item_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = Url::parse(&format!("{}/items/{}", self.url, item_id))?;

        let request =
            authenticated_request_builder(Method::DELETE, &url, api_key).body(Body::empty())?;
        let response = self.client.request(request).await?;

        if response.status() != StatusCode::OK {
            return Err("Failed to delete item".into());
        }

        Ok(())
    }

    pub async fn get_categories(
        &self,
        api_key: &str,
    ) -> Result<Vec<Category>, Box<dyn std::error::Error>> {
        let url = Url::parse(&format!("{}/categories", self.url))?;

        let request =
            authenticated_request_builder(Method::GET, &url, api_key).body(Body::empty())?;
        let response = self.client.request(request).await?;

        let body: hyper::body::Bytes = hyper::body::to_bytes(response.into_body()).await?;
        let json: PageResponse<Category> = serde_json::from_slice(&body)?;

        Ok(json.results)
    }

    pub async fn get_category(
        &self,
        api_key: &str,
        category_id: &str,
    ) -> Result<Category, Box<dyn std::error::Error>> {
        let url = Url::parse(&format!("{}/categories/{}", self.url, category_id))?;

        let request =
            authenticated_request_builder(Method::GET, &url, api_key).body(Body::empty())?;
        let response = self.client.request(request).await?;

        let body: hyper::body::Bytes = hyper::body::to_bytes(response.into_body()).await?;
        let json: Category = serde_json::from_slice(&body)?;

        Ok(json)
    }

    pub async fn get_webhooks(
        &self,
        api_key: &str,
    ) -> Result<Vec<Webhook>, Box<dyn std::error::Error>> {
        let url = Url::parse(&format!("{}/webhooks", self.url))?;

        let request =
            authenticated_request_builder(Method::GET, &url, api_key).body(Body::empty())?;
        let response = self.client.request(request).await?;

        let body: hyper::body::Bytes = hyper::body::to_bytes(response.into_body()).await?;
        let json: PageResponse<Webhook> = serde_json::from_slice(&body)?;

        Ok(json.results)
    }

    pub async fn get_webhook(
        &self,
        api_key: &str,
        webhook_id: &str,
    ) -> Result<Webhook, Box<dyn std::error::Error>> {
        let url = Url::parse(&format!("{}/webhooks/{}", self.url, webhook_id))?;

        let request =
            authenticated_request_builder(Method::GET, &url, api_key).body(Body::empty())?;
        let response = self.client.request(request).await?;

        let body: hyper::body::Bytes = hyper::body::to_bytes(response.into_body()).await?;
        let json: Webhook = serde_json::from_slice(&body)?;

        Ok(json)
    }

    pub async fn create_webhook(
        &self,
        api_key: &str,
        url: &str,
        event: WebhookEvent,
    ) -> Result<Webhook, Box<dyn std::error::Error>> {
        let request_url = Url::parse(&format!("{}/webhooks", self.url))?;

        let create_webhook_request = CreateWebhookRequest {
            url: url.to_string(),
            event,
            headers: None,
        };

        let request = authenticated_request_builder(Method::POST, &request_url, api_key)
            .body(Body::from(serde_json::to_string(&create_webhook_request)?))?;
        let response = self.client.request(request).await?;

        let body: hyper::body::Bytes = hyper::body::to_bytes(response.into_body()).await?;
        let webhook: Webhook = serde_json::from_slice(&body)?;

        Ok(webhook)
    }

    pub async fn delete_webhook(
        &self,
        api_key: &str,
        webhook_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = Url::parse(&format!("{}/webhooks/{}", self.url, webhook_id))?;

        let request =
            authenticated_request_builder(Method::DELETE, &url, api_key).body(Body::empty())?;
        let response = self.client.request(request).await?;

        if response.status() != StatusCode::OK {
            return Err("Failed to delete webhook".into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ITAU_ITEM_ID: &str = "e22c7308-7031-47f0-88a3-462f44d96f70";
    const TEST_SANDBOX_ITEM_ID: &str = "e97238a7-7f5c-4667-8497-5ed8ac4fb509";
    const TEST_WEBHOOK_ID: &str = "6903e8ab-5858-460c-9c6b-2e367ac0d3e9";

    #[test]
    fn can_instantiate_from_env() {
        let result = Client::new_from_env();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn can_instantiate_from_env_with_api_key() {
        let result = Client::new_from_env_with_api_key().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn can_create_connect_token() {
        let (client, api_key) = Client::new_from_env_with_api_key().await.unwrap();
        let connect_token = client.create_connect_token(&api_key).await.unwrap();
        assert_eq!(connect_token.len(), 892);
    }

    #[tokio::test]
    async fn can_get_connectors() {
        let (client, api_key) = Client::new_from_env_with_api_key().await.unwrap();
        let connectors = client.get_connectors(&api_key, false).await.unwrap();
        let connector = connectors.iter().find(|c| c.id == 201);

        match connector {
            Some(connector) => {
                assert_eq!(connector.id, 201);
                assert_eq!(connector.name, "Itaú");
            }
            None => panic!("No connector found"),
        }
    }

    #[tokio::test]
    async fn can_get_connectors_with_sandbox() {
        let (client, api_key) = Client::new_from_env_with_api_key().await.unwrap();
        let connectors = client.get_connectors(&api_key, true).await.unwrap();
        let connector = connectors.iter().find(|c| c.id == 2);

        assert!(connector.is_some());

        match connector {
            Some(connector) => {
                assert_eq!(connector.id, 2);
                assert_eq!(connector.name, "Pluggy Bank");
            }
            None => panic!("No connector found"),
        }
    }

    #[tokio::test]
    async fn can_get_connectors_without_sandbox() {
        let (client, api_key) = Client::new_from_env_with_api_key().await.unwrap();
        let connectors = client.get_connectors(&api_key, false).await.unwrap();
        let connector = connectors.iter().find(|c| c.id == 2);

        assert!(connector.is_none());
    }

    #[tokio::test]
    async fn can_get_connector() {
        let (client, api_key) = Client::new_from_env_with_api_key().await.unwrap();
        let connector = client.get_connector(&api_key, "201").await.unwrap();

        assert_eq!(connector.id, 201);
        assert_eq!(connector.name, "Itaú");
    }

    #[tokio::test]
    async fn can_get_connector_with_sandbox() {
        let (client, api_key) = Client::new_from_env_with_api_key().await.unwrap();
        let connector = client.get_connector(&api_key, "2").await.unwrap();

        assert_eq!(connector.id, 2);
        assert_eq!(connector.name, "Pluggy Bank");
    }

    #[tokio::test]
    async fn can_get_item() {
        let (client, api_key) = Client::new_from_env_with_api_key().await.unwrap();
        let item = client.get_item(&api_key, TEST_ITAU_ITEM_ID).await.unwrap();

        assert_eq!(item.id, TEST_ITAU_ITEM_ID);
        assert!(matches!(item.status, ItemStatus::LoginError));
        assert!(matches!(item.execution_status, ExecutionStatus::Success));
        assert_eq!(item.consecutive_failed_login_attempts, 0);
        assert_eq!(item.connector.id, 201);
        assert_eq!(item.connector.name, "Itaú");
    }

    #[tokio::test]
    async fn can_validate_parameters() {
        let (client, api_key) = Client::new_from_env_with_api_key().await.unwrap();
        let parameters = HashMap::from([("user", "user-ok"), ("password", "password-ok")]);
        let result = client
            .validate_parameters(&api_key, 2, &parameters)
            .await
            .unwrap();

        assert_eq!(result.parameters.len(), 2);

        let user = result.parameters.get_key_value("user");
        assert!(user.is_some());

        match user {
            Some((key, value)) => {
                assert_eq!(key, "user");
                assert_eq!(value, "user-ok");
            }
            _ => panic!("No user parameter found"),
        }

        let password = result.parameters.get_key_value("password");
        assert!(password.is_some());

        match password {
            Some((key, value)) => {
                assert_eq!(key, "password");
                assert_eq!(value, "password-ok");
            }
            _ => panic!("No password parameter found"),
        }

        assert_eq!(result.errors.len(), 0);
    }

    #[tokio::test]
    async fn can_create_item_and_delete() {
        let (client, api_key) = Client::new_from_env_with_api_key().await.unwrap();
        let parameters = HashMap::from([
            ("user".to_string(), "user-ok".to_string()),
            ("password".to_string(), "password-ok".to_string()),
        ]);
        let item = client.create_item(&api_key, 2, &parameters).await.unwrap();

        assert_eq!(item.id.len(), 36);
        assert!(matches!(item.status, ItemStatus::Updating));
        assert!(matches!(item.execution_status, ExecutionStatus::Created));
        assert_eq!(item.consecutive_failed_login_attempts, 0);
        assert_eq!(item.connector.id, 2);
        assert_eq!(item.connector.name, "Pluggy Bank");

        let result = client.delete_item(&api_key, &item.id).await;
        assert!(result.is_ok());

        let item = client.get_item(&api_key, &item.id).await;
        assert!(item.is_err());
    }

    #[tokio::test]
    async fn can_update_item() {
        let (client, api_key) = Client::new_from_env_with_api_key().await.unwrap();
        let parameters = HashMap::from([
            ("user".to_string(), "user-ok".to_string()),
            ("password".to_string(), "password-ok".to_string()),
        ]);
        let item = client
            .update_item(&api_key, TEST_SANDBOX_ITEM_ID, &parameters)
            .await
            .unwrap();

        assert_eq!(item.id, TEST_SANDBOX_ITEM_ID);
        assert!(matches!(item.status, ItemStatus::Updating));
        assert!(matches!(item.execution_status, ExecutionStatus::Created));
        assert_eq!(item.connector.id, 2);
        assert_eq!(item.connector.name, "Pluggy Bank");
    }

    #[tokio::test]
    async fn can_get_categories() {
        let (client, api_key) = Client::new_from_env_with_api_key().await.unwrap();
        let categories = client.get_categories(&api_key).await.unwrap();

        let income_category = categories.iter().find(|c| c.description == "Income");

        assert!(income_category.is_some());
    }

    #[tokio::test]
    async fn can_get_category() {
        let (client, api_key) = Client::new_from_env_with_api_key().await.unwrap();
        let category = client.get_category(&api_key, "01000000").await;
        assert!(category.is_ok());
    }

    #[tokio::test]
    async fn can_get_webhooks() {
        let (client, api_key) = Client::new_from_env_with_api_key().await.unwrap();
        let webhooks = client.get_webhooks(&api_key).await.unwrap();

        assert!(webhooks.len() > 0);
    }

    #[tokio::test]
    async fn can_get_webhook() {
        let (client, api_key) = Client::new_from_env_with_api_key().await.unwrap();
        let webhook = client.get_webhook(&api_key, TEST_WEBHOOK_ID).await.unwrap();

        assert_eq!(webhook.id, TEST_WEBHOOK_ID);
        assert_eq!(webhook.url, "https://some.site/pluggy-notifications");
        assert!(matches!(webhook.event, WebhookEvent::ItemUpdated));
    }

    #[tokio::test]
    async fn can_create_webhook_and_delete() {
        let (client, api_key) = Client::new_from_env_with_api_key().await.unwrap();

        let url = "https://somesite.com/pluggy-notifications";
        let webhook = client
            .create_webhook(&api_key, url, WebhookEvent::ItemUpdated)
            .await
            .unwrap();

        assert_eq!(webhook.id.len(), 36);
        assert_eq!(webhook.url, url);
        assert!(matches!(webhook.event, WebhookEvent::ItemUpdated));

        let result = client.delete_webhook(&api_key, &webhook.id).await;
        assert!(result.is_ok());

        let webhook = client.get_webhook(&api_key, &webhook.id).await;
        assert!(webhook.is_err());
    }
}
