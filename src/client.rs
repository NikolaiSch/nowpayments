use std::fmt::Display;
use std::{collections::HashMap};

use crate::response::{currencies::Currencies, payments::PaymentStatus};
use crate::response::currencies::FullCurrencies;
use crate::response::currencies::SelectedCurrencies;
use crate::response::payments::EstimatedPaymentAmount;
use crate::response::payments::MinPaymentAmount;
use crate::response::payouts::AllPayouts;
use crate::response::payouts::Payouts;
use crate::response::status::Status;
use crate::response::{conversion::SingleConversion, payments::Payment};
use crate::{
    jwt::{JWTJson, JWT},
    response::conversion::AllConversions,
};
use anyhow::{bail, Result};
use reqwest::header;

static BASE_URL: &str = "https://api.nowpayments.io/v1/";
static BASE_SANDBOX_URL: &str = "https://api-sandbox.nowpayments.io/v1/";
static USERAGENT: &str = concat!("rust/nowpayments/", "0.1.0");

pub struct NPClient {
    base_url: &'static str,
    email: Option<String>,
    password: Option<String>,

    jwt: JWT,
    client: reqwest::Client,
}

impl NPClient {
    pub fn new(api_key: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert("X-API-KEY", header::HeaderValue::from_str(api_key).unwrap());

        Self {
            base_url: BASE_URL,
            client: reqwest::ClientBuilder::new()
                .user_agent(USERAGENT)
                .default_headers(headers)
                .build()
                .unwrap(),
            email: None,
            password: None,
            jwt: JWT::new(),
        }
    }

    pub fn new_sandbox(api_key: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert("X-API-KEY", header::HeaderValue::from_str(api_key).unwrap());

        Self {
            base_url: BASE_SANDBOX_URL,
            client: reqwest::ClientBuilder::new()
                .user_agent(USERAGENT)
                .default_headers(headers)
                .build()
                .unwrap(),
            email: None,
            password: None,
            jwt: JWT::new(),
        }
    }

    pub fn set_auth(&mut self, email: String, password: String) {
        self.email = Some(email);
        self.password = Some(password);
    }

    async fn get(&self, endpoint: impl ToString) -> Result<String> {
        let endpoint = format!("{}{}", self.base_url, endpoint.to_string());

        let req = self
            .client
            .get(endpoint)
            .bearer_auth(self.jwt.get().unwrap_or("".to_string()))
            .build()?;

        Ok(self.client.execute(req).await?.text().await?)
    }

    async fn post(
        &self,
        endpoint: impl Display,
        data: HashMap<&'static str, String>,
    ) -> Result<String> {
        let endpoint = format!("{}{}", self.base_url, endpoint);

        let req = self
            .client
            .post(endpoint)
            .json(&data)
            .bearer_auth(self.jwt.get().unwrap_or("".to_string()))
            .build()?;

        Ok(self.client.execute(req).await?.text().await?)
    }

    pub async fn authenticate(&mut self) -> Result<()> {
        if self.email.is_none() || self.password.is_none() {
            bail!("not set username or pass");
        }
        let mut json = HashMap::new();
        json.insert("email", self.email.clone().unwrap());
        json.insert("password", self.password.clone().unwrap());

        let data = self.post("auth", json).await?;
        let jwt: JWTJson = serde_json::from_str(&data)?;
        self.jwt.set(jwt.token);
        Ok(())
    }
}

impl NPClient {
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

    pub async fn get_payment_status(&self, payment_id: impl Display) -> Result<PaymentStatus> {
        if self.jwt.is_expired() {
            bail!("Expired jwt");
        }
        let path = format!("payment/{}", payment_id);
        let req = self.get(path).await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    pub async fn get_list_of_payments(
        &self,
        limit: impl Display,
        page: impl Display,
        sort_by: impl Display,
        order_by: impl Display,
        date_from: impl Display,
        date_to: impl Display,
    ) -> Result<Payment> {
        if self.jwt.is_expired() {
            bail!("Expired jwt");
        }
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

    pub async fn get_conversion_status(
        &self,
        conversion_id: impl Display,
    ) -> Result<SingleConversion> {
        let path = format!("conversion/{}", conversion_id);
        let req = self.get(path).await?;

        Ok(serde_json::from_str(req.as_str())?)
    }

    pub async fn get_conversion_list(&self) -> Result<AllConversions> {
        let path = "conversion".to_string();
        let req = self.get(path).await?;

        Ok(serde_json::from_str(req.as_str())?)
    }
}


pub struct PaymentOpts {
    price_amount: String,
    price_currency: String,
    pay_currency: String,
    ipn_callback_url: String,
    order_id: String,
    order_description: String,
}


impl PaymentOpts {
    pub fn new(
        price_amount: u32,
        price_currency: impl Display,
        pay_currency: impl Display,
        ipn_callback_url: impl Display,
        order_id: impl Display,
        order_description: impl Display,
    ) -> Self {
        PaymentOpts {
            price_amount: price_amount.to_string(),
            price_currency: price_currency.to_string(),
            pay_currency: pay_currency.to_string(),
            ipn_callback_url: ipn_callback_url.to_string(),
            order_id: order_id.to_string(),
            order_description: order_description.to_string(),
        }
    }
}





impl NPClient {
    pub async fn create_payment(&self, opts: PaymentOpts) -> Result<Payment> {
        let mut h = HashMap::new();
        h.insert("price_amount", opts.price_amount.clone());
        h.insert("price_currency", opts.price_currency.clone());
        h.insert("pay_currency", opts.pay_currency.clone());
        h.insert("ipn_callback_url", opts.ipn_callback_url.clone());
        h.insert("order_id", opts.order_id.clone());
        h.insert("order_description", opts.order_description.clone());
        

        let x = self.post("payment", h).await?;

        Ok(serde_json::from_str(x.as_str())?)
    }

    pub fn get_jwt(&self) {
        dbg!(&self.jwt);
    }
}
