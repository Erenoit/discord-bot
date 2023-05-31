//! Spotify structs for deserializing JSON responses.
//!
//! These structs are used to deserialize JSON responses from the [Spotify API].
//! They are used in the [`Song`] struct.
//!
//! [Spotify API]: https://developer.spotify.com/documentation/web-api/
//! [`Song`]: crate::player::song::Song

use serde::{Deserialize, Serialize};

/// General error response from Spotify.
#[derive(Deserialize, Serialize)]
pub struct SpotifyError {
    pub message: String,
}

/// General track response from Spotify.
#[derive(Deserialize, Serialize)]
pub struct SpotifyTrackResponse {
    pub explicit: bool,
    pub name:     String,
}

/// General playlist response from Spotify.
#[derive(Deserialize, Serialize)]
pub struct SpotifyPlaylistResponse {
    pub name:   String,
    pub owner:  SpotifyOwner,
    pub tracks: SpotifyTracks1,
}

/// General album response from Spotify.
#[derive(Deserialize, Serialize)]
pub struct SpotifyAlbumResponse {
    pub name:   String,
    pub tracks: SpotifyTracks2,
}

/// General artist response from Spotify.
#[derive(Deserialize, Serialize)]
pub struct SpotifyArtistTopTracksResponse {
    pub tracks: Vec<SpotifyTrack2>,
}

/// Owner of the track/playlist/album.
#[derive(Deserialize, Serialize)]
pub struct SpotifyOwner {
    pub display_name: Option<String>,
}

/// Track list are returned like this when requested from a playlist URL.
#[derive(Deserialize, Serialize)]
pub struct SpotifyTracks1 {
    pub items:    Vec<SpotifyTrack1>,
    pub limit:    usize,
    pub next:     Option<String>,
    pub offset:   usize,
    pub previous: Option<String>,
    pub total:    usize,
}

/// Track list are returned like this when requested from a album URL.
#[derive(Deserialize, Serialize)]
pub struct SpotifyTracks2 {
    pub items:    Vec<SpotifyTrack2>,
    pub limit:    usize,
    pub next:     Option<String>,
    pub offset:   usize,
    pub previous: Option<String>,
    pub total:    usize,
}

/// Single track are returned like this when requested from a playlist URL.
#[derive(Deserialize, Serialize)]
pub struct SpotifyTrack1 {
    pub track: SpotifyTrackResponse,
}

/// Single track are returned like this when requested from a album or artist
/// URL.
#[derive(Deserialize, Serialize)]
pub struct SpotifyTrack2 {
    pub name: String,
}
