use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------
// Single vide section
// -----------------------------------------------------------------------

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeVideo {
    pub video_details: YoutubeVideoDetails,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeVideoDetails {
    pub title:          String,
    pub video_id:       String,
    pub length_seconds: String,
}

// -----------------------------------------------------------------------
// video with playlist's playlist section
// -----------------------------------------------------------------------

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeVideoPlaylist {
    pub contents: YoutubeVideoPlaylistContents,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeVideoPlaylistContents {
    pub two_column_watch_next_results: TwoColumnWatchNextResults,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct TwoColumnWatchNextResults {
    pub playlist: VideoPlaylist1,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct VideoPlaylist1 {
    pub playlist: VideoPlaylist2,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct VideoPlaylist2 {
    pub contents: Vec<VideoPlaylistContent>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct VideoPlaylistContent {
    pub playlist_panel_video_renderer: PlaylistPanelVideoRenderer,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct PlaylistPanelVideoRenderer {
    pub title:               VideoPlaylistTitle,
    pub navigation_endpoint: VideoPlaylistNavigationEndpoint,
    pub length_text:         VideoPlaylistLengthText,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct VideoPlaylistTitle {
    pub simple_text: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct VideoPlaylistNavigationEndpoint {
    pub watch_endpoint: VideoPlaylistWatchEndpoint,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct VideoPlaylistWatchEndpoint {
    pub video_id: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct VideoPlaylistLengthText {
    pub simple_text: String,
}
