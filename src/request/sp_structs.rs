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
    /// Error message.
    pub message: String,
}

/// General track response from Spotify.
#[derive(Deserialize, Serialize)]
pub struct SpotifyTrackResponse {
    /// Is the track explicit?
    pub explicit: bool,
    /// Track name.
    pub name:     String,
}

/// General playlist response from Spotify.
#[derive(Deserialize, Serialize)]
pub struct SpotifyPlaylistResponse {
    /// Playlist name.
    pub name:   String,
    /// Playlist owner.
    pub owner:  SpotifyOwner,
    /// Playlist tracks.
    pub tracks: SpotifyTracks1,
}

/// General album response from Spotify.
#[derive(Deserialize, Serialize)]
pub struct SpotifyAlbumResponse {
    /// Album name.
    pub name:   String,
    /// Album tracks.
    pub tracks: SpotifyTracks2,
}

/// General artist response from Spotify.
#[derive(Deserialize, Serialize)]
pub struct SpotifyArtistTopTracksResponse {
    /// Artist top tracks.
    pub tracks: Vec<SpotifyTrack2>,
}

/// Owner of the track/playlist/album.
#[derive(Deserialize, Serialize)]
pub struct SpotifyOwner {
    /// Owner display name.
    pub display_name: Option<String>,
}

/// Track list are returned like this when requested from a playlist URL.
#[derive(Deserialize, Serialize)]
pub struct SpotifyTracks1 {
    /// List of tracks.
    pub items:    Vec<SpotifyTrack1>,
    /// Number of tracks.
    pub limit:    usize,
    /// Next URL.
    pub next:     Option<String>,
    /// Offset.
    pub offset:   usize,
    /// Previous URL.
    pub previous: Option<String>,
    /// Total number of tracks.
    pub total:    usize,
}

/// Track list are returned like this when requested from a album URL.
#[derive(Deserialize, Serialize)]
pub struct SpotifyTracks2 {
    /// List of tracks.
    pub items:    Vec<SpotifyTrack2>,
    /// Number of tracks.
    pub limit:    usize,
    /// Next URL.
    pub next:     Option<String>,
    /// Offset.
    pub offset:   usize,
    /// Previous URL.
    pub previous: Option<String>,
    /// Total number of tracks.
    pub total:    usize,
}

/// Single track are returned like this when requested from a playlist URL.
#[derive(Deserialize, Serialize)]
pub struct SpotifyTrack1 {
    /// Track information.
    pub track: SpotifyTrackResponse,
}

/// Single track are returned like this when requested from a album or artist
/// URL.
#[derive(Deserialize, Serialize)]
pub struct SpotifyTrack2 {
    /// Track name.
    pub name: String,
}
