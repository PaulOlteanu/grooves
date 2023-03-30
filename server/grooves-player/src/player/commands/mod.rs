// TODO: Command to change device id?

use grooves_entity::playlist;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayData {
    pub playlist: playlist::Model,
    pub element_index: Option<usize>,
    pub song_index: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Command {
    Play(PlayData),
    Pause,
    Resume,
    NextSong,
    PrevSong,
    NextElement,
    PrevElement,
    AddToQueue,
    RemoveFromQueue,
    Exit,
}
