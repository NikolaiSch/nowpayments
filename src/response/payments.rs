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
