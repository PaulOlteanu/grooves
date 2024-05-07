use rspotify::Token;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub spotify_id: String,
    pub token: Option<Token>,
}
