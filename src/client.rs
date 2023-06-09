use std::fmt::Display;
use std::fmt::format;

use reqwest::Client;
use reqwest::Request;
use anyhow::Result;
use reqwest::header;
use crate::response::currencies::Currencies;
use crate::response::currencies::FullCurrencies;
use crate::response::status::Status;




static BASE_URL: &str = "https://api.nowpayments.io/v1/";
static BASE_SANDBOX_URL: &str = "https://api-sandbox.nowpayments.io/v1/";
static USERAGENT: &str = concat!("rust/nowpayments/", "0.1.0");


pub struct NPClient {
    api_key: String,
    base_url: &'static str,

    client: reqwest::Client
}

impl NPClient {
    pub fn new(api_key: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert("X-API-KEY", header::HeaderValue::from_str(api_key).unwrap());
        
        Self {
            api_key: api_key.to_string(),
            base_url: BASE_URL,
            client: reqwest::ClientBuilder::new().user_agent(USERAGENT).default_headers(headers).build().unwrap()
        }
    }

    pub fn new_sandbox(api_key: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert("X-API-KEY", header::HeaderValue::from_str(api_key).unwrap());

        Self {
            api_key: api_key.to_string(),
            base_url: BASE_SANDBOX_URL,
            client: reqwest::ClientBuilder::new().user_agent(USERAGENT).default_headers(headers).build().unwrap()
        }
    }

    async fn get(&self, endpoint: impl ToString) -> Result<String> {
        let endpoint = format!("{}{}", self.base_url, endpoint.to_string());

        let req = self.client.get(endpoint).build()?;

        Ok(self.client.execute(req).await?.text().await?)

    }

    pub async fn status(&self) -> Result<Status> {
        let req = self.get("status").await?;

        let status: Status = serde_json::from_str(req.as_str())?;

        Ok(status)
    }

    pub async fn currencies(&self) -> Result<Currencies> {
        let req = self.get("currencies").await?;

        let currencies: Currencies = serde_json::from_str(req.as_str())?;
       
        Ok(currencies)
    }

    pub async fn full_currencies(&self) -> Result<FullCurrencies> {
        let req = self.get("full-currencies").await?;

        let currencies: FullCurrencies = serde_json::from_str(req.as_str())?;
        
        Ok(currencies)
    }

    pub async fn selected_currencies(&self) -> Result<Currencies> {
        let req = self.get("merchant/coins").await?;

        let currencies: Currencies = serde_json::from_str(req.as_str())?;
       
        Ok(currencies)
    }
    // TODO
    pub async fn min_payment_amount(&self, from: impl Display, to: impl Display) -> Result<Currencies> {
        let path = format!("min-amount?currency_from={}&currency_to={}", from, to);
        let req = self.get(path).await?;

        let currencies: Currencies = serde_json::from_str(req.as_str())?;
       
        Ok(currencies)
    }
    // TODO
    pub async fn get_estimated_price(&self, amount: impl Display, from: impl Display, to: impl Display) -> Result<Currencies> {
        let path = format!("estimate?amount={}&currency_from={}&currency_to={}", amount, from, to);
        let req = self.get(path).await?;

        let currencies: Currencies = serde_json::from_str(req.as_str())?;
       
        Ok(currencies)
    }
}