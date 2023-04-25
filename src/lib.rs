use hyper::http::request::Builder;
use hyper::{Body, Client as HyperClient, Method, Request};
use hyper_tls::HttpsConnector;

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

pub fn authenticated_request_builder(method: Method, url: &str, api_key: &str) -> Builder {
    Request::builder()
        .method(method)
        .uri(url)
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
        let url = format!("{}/auth", self.url);

        let payload = AuthRequest {
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            non_expiring: None,
        };

        let json_payload = serde_json::to_string(&payload)?;

        let request = Request::builder()
            .method(Method::POST)
            .uri(url)
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
        let url = format!("{}/connect_token", self.url);
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
    ) -> Result<Vec<Connector>, Box<dyn std::error::Error>> {
        let url = format!("{}/connectors", self.url);
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
        let url = format!("{}/connectors/{}", self.url, connector_id);
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
        let url = format!("{}/items/{}", self.url, item_id);
        let request =
            authenticated_request_builder(Method::GET, &url, api_key).body(Body::empty())?;
        let response = self.client.request(request).await?;

        let body: hyper::body::Bytes = hyper::body::to_bytes(response.into_body()).await?;
        let json: Item = serde_json::from_slice(&body)?;

        Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_instantiate_from_env() {
        Client::new_from_env().unwrap();
    }

    #[tokio::test]
    async fn can_instantiate_from_env_with_api_key() {
        Client::new_from_env_with_api_key().await.unwrap();
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
        let connectors = client.get_connectors(&api_key).await.unwrap();

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
    async fn can_get_connector() {
        let (client, api_key) = Client::new_from_env_with_api_key().await.unwrap();
        let connector = client.get_connector(&api_key, "201").await.unwrap();

        assert_eq!(connector.id, 201);
        assert_eq!(connector.name, "Itaú");
    }

    #[tokio::test]
    async fn can_get_item() {
        let (client, api_key) = Client::new_from_env_with_api_key().await.unwrap();
        let item = client.get_item(&api_key, "123").await.unwrap();

        assert_eq!(item.id, "123");
    }
}
