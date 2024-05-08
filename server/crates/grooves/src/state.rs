use std::collections::HashMap;
use std::sync::Mutex;

use grooves_model::User;
use grooves_player::manager::PlayerManager;
use sqlx::PgPool;

pub struct State {
    pub db_pool: PgPool,
    pub player_manager: PlayerManager,
    pub sse_tokens: Mutex<HashMap<String, User>>,
}
