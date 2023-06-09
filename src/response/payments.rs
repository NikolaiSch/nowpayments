use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct MinPaymentAmount {
    currency_from: String,
    currency_to: String,
    min_amount: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EstimatedPaymentAmount {
    currency_from: String,
    currency_to: String,
    amount_from: f64,
    estimated_amount: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payment {
    pub payment_id: String,
    pub payment_status: String,
    pub pay_address: String,
    pub price_amount: f64,
    pub price_currency: String,
    pub pay_amount: f64,
    pub pay_currency: String,
    pub order_id: String,
    pub order_description: String,
    pub purchase_id: String,
}

