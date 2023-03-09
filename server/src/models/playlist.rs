use rspotify::model::TrackId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistElement {
    pub name: String,
    pub songs: Vec<Song>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Song {
    pub name: String,
    pub spotify_id: TrackId<'static>,
}
