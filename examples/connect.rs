use jsonwebtoken::decode_header;
use pluggy::Client;

#[tokio::main]
async fn main() {
    let (client, api_key) = Client::new_from_env_with_api_key().await.unwrap();

    let connect_token = client.create_connect_token(&api_key).await.unwrap();
    let header = decode_header(&connect_token).unwrap();

    print!("Created a new connect token with header {:?}", header)
}
