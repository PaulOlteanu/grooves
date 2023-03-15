// TODO: Command to change device id?

use phonos_entity::playlist;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Command {
    Play {
        playlist: playlist::Model,
        element_index: Option<usize>,
        song_index: Option<usize>,
    },
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
