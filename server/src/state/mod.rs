use std::collections::HashMap;
use std::sync::Mutex;

use grooves_entity::user;
use grooves_player::manager::PlayerManager;
use sea_orm::DatabaseConnection;

pub struct State {
    pub db_pool: DatabaseConnection,
    pub player_manager: PlayerManager,
    pub sse_tokens: Mutex<HashMap<String, user::Model>>,
}
