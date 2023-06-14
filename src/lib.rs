pub mod client;
pub mod jwt;
pub mod response;

#[cfg(test)]
mod test {
    use crate::client::PaymentOpts;

    use super::client::NPClient;
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
        NPClient::new(config.api.as_str())
    }

    fn sandbox_client() -> NPClient {
        let config = parse_config();
        NPClient::new(config.sandboxapi.as_str())
    }

    #[test]
    fn verify_client() {
        client();
    }

    #[test]
    fn verify_sandbox_client() {
        sandbox_client();
    }

    #[tokio::test]
    async fn get_status() {
        let mut c = client();

        let status = c.status().await.unwrap();

        assert_eq!(status.message, "OK".to_string())
    }

    #[tokio::test]
    async fn get_currencies() {
        let mut c = client();

        // panics if not error
        c.get_currencies().await.unwrap();
    }

    #[tokio::test]
    async fn get_full_currencies() {
        let mut c = client();

        // panics if not error
        c.get_full_currencies().await.unwrap();
    }

    #[tokio::test]
    async fn get_checked_currencies() {
        let mut c = client();

        // panics if not error
        c.get_checked_currencies().await.unwrap();
    }

    #[tokio::test]
    async fn get_min_payment_amount() {
        let mut c = client();

        // panics if not error
        c.get_min_payment_amount("eth", "btc").await.unwrap();
    }

    #[tokio::test]
    async fn get_estimate_price() {
        let mut c = client();

        // panics if not error
        c.get_estimated_price(2000, "btc", "eth").await.unwrap();
    }

    #[tokio::test]
    async fn authentication() {
        let conf = parse_config();
        let mut c = client();
        c.set_auth(conf.email, conf.password);
        // panics if not error
        c.authenticate().await.unwrap();

        c.get_payout_list().await.unwrap();
    }

    #[tokio::test]
    async fn create_payment() {
        let payment = PaymentOpts::new(100, "GBP", "BTC", "http://google.com/", "x", "test order");

        let mut c = client();

        c.create_payment(payment).await.unwrap();
    }
}
