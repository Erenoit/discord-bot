//! Spotify structs for deserializing JSON responses.
//!
//! These structs are used to deserialize JSON responses from the [Spotify API].
//! They are used in the [`Song`] struct.
//!
//! [Spotify API]: https://developer.spotify.com/documentation/web-api/
//! [`Song`]: crate::player::song::Song

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SpotifyError {
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct SpotifyToken {
    pub access_token: String,
    pub expires_in:   u64,
}

#[derive(Deserialize, Serialize)]
pub struct SpotifyTrack {
    pub explicit: bool,
    pub name:     String,
    pub artists:  Vec<SpotifyArtist>,
}

#[derive(Deserialize, Serialize)]
pub struct SpotifyPlaylist {
    pub name:   String,
    pub owner:  SpotifyOwner,
    pub tracks: SpotifyTracks1,
}

#[derive(Deserialize, Serialize)]
pub struct SpotifyAlbum {
    pub name:   String,
    pub tracks: SpotifyTracks2,
}

#[derive(Deserialize, Serialize)]
pub struct SpotifyArtistTopTracks {
    pub tracks: Vec<SpotifyTrack2>,
}

#[derive(Deserialize, Serialize)]
pub struct SpotifyOwner {
    pub display_name: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct SpotifyTracks1 {
    pub items:    Vec<SpotifyTrack1>,
    pub limit:    usize,
    pub next:     Option<String>,
    pub offset:   usize,
    pub previous: Option<String>,
    pub total:    usize,
}

#[derive(Deserialize, Serialize)]
pub struct SpotifyTracks2 {
    pub items:    Vec<SpotifyTrack2>,
    pub limit:    usize,
    pub next:     Option<String>,
    pub offset:   usize,
    pub previous: Option<String>,
    pub total:    usize,
}

#[derive(Deserialize, Serialize)]
pub struct SpotifyTrack1 {
    pub track: SpotifyTrack,
}

#[derive(Deserialize, Serialize)]
pub struct SpotifyTrack2 {
    pub name:    String,
    pub artists: Vec<SpotifyArtist>,
}

#[derive(Deserialize, Serialize)]
pub struct SpotifyArtist {
    pub name: String,
}
