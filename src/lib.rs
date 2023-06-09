pub mod client;

#[cfg(test)]
mod test {
    use super::client::NPClient;
    use envy;
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    struct Config {
        api: String,
        sandboxapi: String,
    }

    fn parse_config() -> Config {
        dotenvy::dotenv().unwrap();
        envy::prefixed("NP_").from_env().unwrap()
    }

    fn client() -> NPClient {
        let config = parse_config();
        NPClient::new(config.api)
    }

    fn sandbox_client() -> NPClient {
        let config = parse_config();
        NPClient::new(config.sandboxapi)
    }

    #[test]
    fn verify_client() {
        client();
    }

    #[test]
    fn verify_sandbox_client() {
        sandbox_client();
    }
}
