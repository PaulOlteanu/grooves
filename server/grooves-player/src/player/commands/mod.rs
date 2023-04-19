use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Command {
    Play {
        playlist_id: i32,
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
