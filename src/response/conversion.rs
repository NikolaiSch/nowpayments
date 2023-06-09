use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleConversion {
    pub result: Result,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllConversions {
    pub result: Vec<Result>,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result {
    pub id: String,
    pub status: String,
    pub from_currency: String,
    pub to_currency: String,
    pub from_amount: f64,
    pub to_amount: Option<f64>,
    pub created_at: String,
    pub updated_at: String,
}
