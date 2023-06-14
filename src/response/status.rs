use miniserde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub message: String,
}
