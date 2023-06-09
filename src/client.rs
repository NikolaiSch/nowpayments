use std::fmt::format;
use std::fmt::Display;

use crate::response::conversion::AllConversions;
use crate::response::conversion::SingleConversion;
use crate::response::currencies::Currencies;
use crate::response::currencies::FullCurrencies;
use crate::response::currencies::SelectedCurrencies;
use crate::response::payments::EstimatedPaymentAmount;
use crate::response::payments::MinPaymentAmount;
use crate::response::payouts::AllPayouts;
use crate::response::payouts::Payouts;
use crate::response::status::Status;
use anyhow::Result;
use reqwest::header;
use reqwest::Client;
use reqwest::Request;
use serde_json::Value;

static BASE_URL: &str = "https://api.nowpayments.io/v1/";
static BASE_SANDBOX_URL: &str = "https://api-sandbox.nowpayments.io/v1/";
static USERAGENT: &str = concat!("rust/nowpayments/", "0.1.0");

pub struct NPClient {
    api_key: String,
    base_url: &'static str,

    client: reqwest::Client,
}

impl NPClient {
    pub fn new(api_key: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert("X-API-KEY", header::HeaderValue::from_str(api_key).unwrap());

        Self {
            api_key: api_key.to_string(),
            base_url: BASE_URL,
            client: reqwest::ClientBuilder::new()
                .user_agent(USERAGENT)
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }

    pub fn new_sandbox(api_key: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert("X-API-KEY", header::HeaderValue::from_str(api_key).unwrap());

        Self {
            api_key: api_key.to_string(),
            base_url: BASE_SANDBOX_URL,
            client: reqwest::ClientBuilder::new()
                .user_agent(USERAGENT)
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }

    async fn get(&self, endpoint: impl ToString) -> Result<String> {
        let endpoint = format!("{}{}", self.base_url, endpoint.to_string());

        let req = self.client.get(endpoint).build()?;

        Ok(self.client.execute(req).await?.text().await?)
    }

    pub async fn status(&self) -> Result<Status> {
        let req = self.get("status").await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    pub async fn get_currencies(&self) -> Result<Currencies> {
        let req = self.get("currencies").await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    pub async fn get_full_currencies(&self) -> Result<FullCurrencies> {
        let req = self.get("full-currencies").await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    pub async fn get_checked_currencies(&self) -> Result<SelectedCurrencies> {
        let req = self.get("merchant/coins").await?;

        Ok(serde_json::from_str(req.as_str())?)
    }
    // TODO
    pub async fn get_min_payment_amount(
        &self,
        from: impl Display,
        to: impl Display,
    ) -> Result<MinPaymentAmount> {
        let path = format!("min-amount?currency_from={}&currency_to={}", from, to);
        let req = self.get(path).await?;

        Ok(serde_json::from_str(req.as_str())?)
    }
    // TODO
    pub async fn get_estimated_price(
        &self,
        amount: impl Display,
        from: impl Display,
        to: impl Display,
    ) -> Result<EstimatedPaymentAmount> {
        let path = format!(
            "estimate?amount={}&currency_from={}&currency_to={}",
            amount, from, to
        );
        let req = self.get(path).await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    pub async fn get_payment_status(&self, payment_id: impl Display) -> Result<Currencies> {
        let path = format!("payment/{}", payment_id);
        let req = self.get(path).await?;

        Ok(serde_json::from_str(req.as_str())?)
    }
    
    // TODO: need jwt
    pub async fn get_list_of_payments(
        &self,
        limit: impl Display,
        page: impl Display,
        sort_by: impl Display,
        order_by: impl Display,
        date_from: impl Display,
        date_to: impl Display,
    ) -> Result<Currencies> {
        let path = format!(
            "payment/?limit={}&page={}&sortBy={}&orderBy={}&dateFrom={}&dateTo={}",
            limit, page, sort_by, order_by, date_from, date_to
        );
        let req = self.get(path).await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    // TODO
    pub async fn get_balance(&self) -> Result<Status> {
        let req = self.get("balance").await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    pub async fn get_payout_status(&self, payout_id: impl Display) -> Result<Payouts> {
        let path = format!("payout/{}", payout_id);
        let req = self.get(path).await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    pub async fn get_payout_list(&self) -> Result<AllPayouts> {
        let req = self.get("payout").await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    pub async fn get_conversion_status(&self, conversion_id: impl Display) -> Result<SingleConversion> {
        let path = format!("conversion/{}", conversion_id);
        let req = self.get(path).await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    pub async fn get_conversion_list(&self) -> Result<AllConversions> {
        let path = format!("conversion");
        let req = self.get(path).await?;

        Ok(serde_json::from_str(req.as_str())?)
    }
}
