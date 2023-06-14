use std::time::{Instant, Duration};

use anyhow::bail;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct JWTJson {
    pub token: String,
}

#[derive(Debug)]
pub struct JWT {
    token: Option<String>,
    time: Option<Instant>
}

impl JWT {
    pub fn new() -> Self {
        Self { token: None, time: None }
    } 

    pub fn is_expired(&mut self) -> bool {
        if self.time.is_none() {
            return true;
        }
        let duration = Instant::now() - self.time.unwrap();

        if duration >= Duration::from_secs(300) {
            self.token = None;
            self.time = None;
            return true;
        }

        false
    }

    pub fn get(&mut self) -> anyhow::Result<String> {
        if self.is_expired() {
            bail!("expired jwt");
        }

        if let Some(x) = self.token.clone() {
            return Ok(x);
        }

        bail!("No jwt yet");
    }

    pub fn set(&mut self, token: String) {
        self.time = Some(Instant::now());
        self.token = Some(token);
    }
}

impl Default for JWT {
    fn default() -> Self {
        JWT::new()
    }
}