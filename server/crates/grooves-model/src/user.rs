use rspotify::Token;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub spotify_id: String,
    #[sqlx(json)]
    pub token: Option<Token>,
}
