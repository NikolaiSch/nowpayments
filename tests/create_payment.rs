use anyhow::{bail, Result};
use nowpayments::client::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Config {
    api: String,
    sandboxapi: String,
    email: String,
    password: String,
}

fn parse_config() -> Config {
    dotenvy::dotenv().unwrap();
    envy::prefixed("NP_").from_env().unwrap()
}

fn client() -> NPClient {
    let config = parse_config();
    let mut client = NPClient::new(config.api.as_str());
    client.set_auth(config.email, config.password);
    client
}

// fn sandbox_client() -> NPClient {
//     let config = parse_config();
//     let mut client = NPClient::new(config.sandboxapi.as_str());
//     client.set_auth(config.email, config.password);
//     client
// }

async fn create_payment(mut client: NPClient) -> Result<()> {
    let id = uuid::Uuid::new_v4().to_string();
    let payment = PaymentOpts::new(100, "GBP", "BTC", "http://google.com/", &id, "test order");
    client.authenticate().await?;
    let order = client.create_payment(payment).await?;
    dbg!(&order);
    let status = client.get_payment_status(order.payment_id).await;
    dbg!(&status);

    let status = status?;

    let valid = status.order_id == id;
    if valid {
        return Ok(());
    }
    bail!(
        "Not matching uuid | correct: {}| received: {}",
        id.to_string(),
        status.order_id
    );
}

#[tokio::test]
async fn normal_api() -> Result<()> {
    let client = client();
    create_payment(client).await?;
    Ok(())
}

// #[tokio::test]
// async fn sandbox_api() -> Result<()> {
//     let client = sandbox_client();
//     create_payment(client).await?;
//     Ok(())
// }
