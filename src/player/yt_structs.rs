//! Yu-ouTube struct to scrape search result
//!
//! It may have other struct to scrape other things in the future.
//!
//! This file is probably completely unmaintainable. It will be really hard to
//! change anything if youtube decides to change their datastructure.

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

/// Entry of `ytInitialData` variable in the HTML source code.
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeLink {
    /// `contents` of `ytInitialData` variable in the HTML source code.
    pub contents: YoutubeWatchContents,
}

/// Container
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeWatchContents {
    /// Container
    pub two_column_watch_next_results: YoutubeWatchTwoColumn,
}

/// Container
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeWatchTwoColumn {
    /// Video
    // pub results:  YoutubeResultsResults,
    // Playlist
    pub playlist: Option<YoutubePlaylist>,
}

/// Container
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubePlaylist {
    /// Container
    pub playlist: YoutubePlaylistPlaylist,
}

/// Container
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubePlaylistPlaylist {
    /// Container
    pub contents: Vec<YoutubePlaylistContent>,
}

/// Container
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubePlaylistContent {
    /// Title
    pub title:          YoutubePlaylistTitle,
    /// Length
    pub length_text:    YoutubeLengthText,
    /// Other informations
    pub watch_endpoint: YoutubeWatchEndpoint,
}

/// Title Container for Playlist
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubePlaylistTitle {
    /// Title
    pub simple_text: String,
}

/// Information about videa
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeWatchEndpoint {
    /// Video id
    pub video_id: String,
}

/// Container
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeResultsResults {
    /// Container
    pub results: YoutubeResultsContents,
}

/// Container
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeResultsContents {
    /// Container
    pub contents: Vec<YoutubeResultsContent>,
}

/// Container
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeResultsContent {
    /// Container
    pub video_primary_info_renderer: Option<YoutubeVideoPrimaryInfoRenderer>,
}

/// Container
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeVideoPrimaryInfoRenderer {
    /// Container
    pub title:       YoutubeTitle,
    /// Container
    pub length_text: YoutubeLengthText,
}

/// Entry of `ytInitialData` variable in the HTML source code.
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSearch {
    /// `contents` of `ytInitialData` variable in the HTML source code.
    pub contents: YoutubeSearchContents,
}

/// Container
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSearchContents {
    /// Container
    pub two_column_search_results_renderer: YoutubeSearchTwoColumn,
}

/// Container
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSearchTwoColumn {
    /// Container
    pub primary_contents: YoutubeSearchPrimaryContents,
}

/// Container
///
/// Only difference from `YoutubePrimaryContentsSelection` is the field name
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSearchPrimaryContents {
    /// Container
    pub section_list_renderer: YoutubeSearchListRenderer,
}

/// Container
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSearchListRenderer {
    /// Container
    pub contents: Vec<YoutubeSearchListRendererContents>,
}

/// Container
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSearchListRendererContents {
    /// Container
    pub item_section_renderer: Option<YoutubeSearchItemSectionRenderer>,
}

/// A struct that holds list of videos
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSearchItemSectionRenderer {
    /// A list of videos
    pub contents: Vec<YoutubeSearchItemSectionRendererContents>,
}

/// Struct that hold a single video
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSearchItemSectionRendererContents {
    /// A single video
    pub video_renderer: Option<YoutubeSearchVideoRenderer>,
}

/// Information about a video
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSearchVideoRenderer {
    /// Id of the video
    pub video_id:    String,
    /// Title of the video
    pub title:       YoutubeTitle,
    /// Length of the video
    pub length_text: YoutubeLengthText,
}

/// List of titles of a videa
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeTitle {
    /// List of titles of a videa
    pub runs: VecDeque<YoutubeTitleRun>,
}

/// Single title of a video
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeTitleRun {
    /// Single title of a video
    pub text: String,
}

/// Length of a video
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeLengthText {
    /// length of the song as string
    pub simple_text: String,
}
