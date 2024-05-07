use rspotify::model::TrackId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Playlist {
    pub id: i32,
    pub name: String,
    pub owner_id: i32,
    pub elements: Vec<PlaylistElement>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaylistElement {
    pub name: String,
    pub image_url: String,
    pub artists: String,
    pub songs: Vec<Song>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Song {
    pub name: String,
    pub image_url: String,
    pub artists: String,
    pub spotify_id: TrackId<'static>,
}
