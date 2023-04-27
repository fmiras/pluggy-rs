use std::collections::HashMap;

use pluggy::Client;

#[tokio::main]
async fn main() {
    let (client, api_key) = Client::new_from_env_with_api_key().await.unwrap();
    let parameters = HashMap::from([
        ("user".to_string(), "user-ok".to_string()),
        ("password".to_string(), "password-ok".to_string()),
    ]);
    let item = client.create_item(&api_key, 2, &parameters).await.unwrap();

    println!("Item: {:?}", item);
}
