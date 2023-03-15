use axum::extract::ws::Message;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub enum OutgoingMessage {
    RequestToken,
    PlayerState,
    Reauthorize,
}

impl Into<Message> for OutgoingMessage {
    fn into(self) -> Message {
        let as_str = serde_json::to_string(&self).unwrap();
        Message::Text(as_str)
    }
}
