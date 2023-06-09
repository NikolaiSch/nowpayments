use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

pub type Payouts = Vec<Payout>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payout {
    pub id: String,
    pub address: String,
    pub currency: String,
    pub amount: String,
    pub batch_withdrawal_id: String,
    pub status: String,
    pub extra_id: Option<String>,
    pub hash: Option<String>,
    pub error: Value,
    pub is_request_payouts: bool,
    pub ipn_callback_url: Option<String>,
    pub unique_external_id: Option<String>,
    pub payout_description: Option<String>,
    pub created_at: Option<String>,
    pub requested_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllPayouts {
    pub payouts: Vec<Payout>,
}
