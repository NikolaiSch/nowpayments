
static BASE_URL: &str = "https://api.nowpayments.io/v1/{}";
static BASE_SANDBOX_URL: &str = "https://api-sandbox.nowpayments.io/v1/{}";
static USERAGENT: &str = concat!("rust/nowpayments/", "0.1.0");


pub struct NPClient {
    api_key: String,
    base_url: &'static str,

    client: reqwest::Client
}

impl NPClient {
    pub fn new(api_key: impl ToString) -> Self {
        Self {
            api_key: api_key.to_string(),
            base_url: BASE_URL,
            client: reqwest::ClientBuilder::new().user_agent(USERAGENT).build().unwrap()
        }
    }

    pub fn new_sandbox(api_key: impl ToString) -> Self {
        Self {
            api_key: api_key.to_string(),
            base_url: BASE_SANDBOX_URL,
            client: reqwest::ClientBuilder::new().user_agent(USERAGENT).build().unwrap()
        }
    }
}