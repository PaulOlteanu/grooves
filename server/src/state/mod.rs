use std::collections::HashMap;
use std::sync::Mutex;

use grooves_entity::user;
use grooves_player::manager::PlayerManager;
use sea_orm::DatabaseConnection;

// TODO: We need some way to delete player connections when the player closes
pub struct State {
    pub db: DatabaseConnection,
    pub player_manager: PlayerManager,
    pub sse_tokens: Mutex<HashMap<String, user::Model>>,
}
