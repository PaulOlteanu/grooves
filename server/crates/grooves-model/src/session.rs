use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
}
