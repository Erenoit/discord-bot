use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct SpotifyError {
    pub message: String
}

#[derive(Deserialize, Serialize)]
pub struct SpotifyTrackResponse {
    pub explicit: bool,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct SpotifyPlaylistResponse {
    pub name: String,
    pub owner: SpotifyOwner,
    pub tracks: SpotifyTracks1,
}

#[derive(Deserialize, Serialize)]
pub struct SpotifyAlbumResponse {
    pub name: String,
    pub tracks: SpotifyTracks2,
}

#[derive(Deserialize, Serialize)]
pub struct SpotifyArtistTopTracksResponse {
    pub tracks: Vec<SpotifyTrack2>
}

#[derive(Deserialize, Serialize)]
pub struct SpotifyOwner {
    pub display_name: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct SpotifyTracks1 {
    pub items: Vec<SpotifyTrack1>,
    pub limit: usize,
    pub next: Option<String>,
    pub offset: usize,
    pub previous: Option<String>,
    pub total: usize
}

#[derive(Deserialize, Serialize)]
pub struct SpotifyTracks2 {
    pub items: Vec<SpotifyTrack2>,
    pub limit: usize,
    pub next: Option<String>,
    pub offset: usize,
    pub previous: Option<String>,
    pub total: usize
}

#[derive(Deserialize, Serialize)]
pub struct SpotifyTrack1 {
    pub track: SpotifyTrackResponse
}

#[derive(Deserialize, Serialize)]
pub struct  SpotifyTrack2 {
    pub name: String,
}

