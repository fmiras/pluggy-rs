use pluggy::Client;

#[tokio::main]
async fn main() {
    let (client, api_key) = Client::new_from_env_with_api_key().await.unwrap();

    let connectors = client.get_connectors(&api_key).await.unwrap();

    for connector in connectors {
        println!("Connector #{}: {}", connector.id, connector.name);
    }
}
