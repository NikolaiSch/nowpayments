use serde::Deserialize;
use serde::Serialize;


#[derive(Serialize, Deserialize)]
pub struct Status {
    pub message: String
}