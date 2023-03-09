// TODO: This file should be somewhere else
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct SpotifyCreds {
    pub access_token: String,
    pub refresh_token: Option<String>,
}

impl SpotifyCreds {
    pub fn new(access_token: String, refresh_token: Option<String>) -> Self {
        Self {
            access_token,
            refresh_token,
        }
    }
}
