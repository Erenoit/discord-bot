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

// -----------------------------------------------------------------------
// playlist section
// -----------------------------------------------------------------------

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubePlaylist {
    pub contents: YoutubePlaylistContents,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubePlaylistContents {
    pub two_column_browse_results_renderer: TwoColumnBrowseResultsRenderer,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct TwoColumnBrowseResultsRenderer {
    // TODO: is there more tabs????
    pub tabs: Vec<Tab>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct Tab {
    pub tab_renderer: Option<TabRenderer>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct TabRenderer {
    pub content: TabRendererContent,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct TabRendererContent {
    pub section_list_renderer: SectionListRenderer,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct SectionListRenderer {
    pub contents: Vec<SectionListContent>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct SectionListContent {
    pub item_section_renderer: Option<ItemSelectionRenderer>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct ItemSelectionRenderer {
    pub contents: Vec<ItemSelectionContent>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct ItemSelectionContent {
    pub playlist_video_list_renderer: Option<PlaylistVideoListRenderer>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct PlaylistVideoListRenderer {
    pub contents: Vec<PlaylistVideoRendererContent>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct PlaylistVideoRendererContent {
    pub playlist_video_renderer: PlaylistVideoRenderer,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct PlaylistVideoRenderer {
    pub title:       PlaylistVideoTitle,
    pub video_id:    String,
    pub length_text: PlaylistVideoLengthText,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct PlaylistVideoTitle {
    pub runs: Vec<PlaylistVideoTitleRun>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct PlaylistVideoTitleRun {
    pub text: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct PlaylistVideoLengthText {
    pub simple_text: String,
}
