//! Yu-ouTube struct to scrape search result
//!
//! It may have other struct to scrape other things in the future.

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

/// Entry of `ytInitialData` variable in the HTML source code.
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSearchResult {
    /// `contents` of `ytInitialData` variable in the HTML source code.
    pub contents: YoutubeSearchResultContents,
}

/// `contents` of `ytInitialData` variable in the HTML source code.
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSearchResultContents {
    /// Container
    pub two_column_search_results_renderer: YoutubeSearchResultRenderer,
}

/// Container
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSearchResultRenderer {
    /// Container
    pub primary_contents: YoutubeSearchResultPrimaryContents,
}

/// Container
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSearchResultPrimaryContents {
    /// Container
    pub section_list_renderer: YoutubeSearchResultSectionListRenderer,
}

/// Container
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSearchResultSectionListRenderer {
    /// Container
    pub contents: Vec<YoutubeSearchResultSectionListRendererContents>,
}

/// Container
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSearchResultSectionListRendererContents {
    /// Container
    pub item_section_renderer: Option<YoutubeSearchResultItemSectionRenderer>,
}

/// A struct that holds list of videos
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSearchResultItemSectionRenderer {
    /// A list of videos
    pub contents: Vec<YoutubeSearchResultItemSectionRendererContents>,
}

/// Struct that hold a single video
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSearchResultItemSectionRendererContents {
    /// A single video
    pub video_renderer: Option<YoutubeSearchResultVideoRenderer>,
}

/// Information about a video
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSearchResultVideoRenderer {
    /// Id of the video
    pub video_id:    String,
    /// Title of the video
    pub title:       YoutubeSearchResultTitle,
    /// Length of the video
    pub length_text: YoutubeSearchResultLengthText,
}

/// List of titles of a videa
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSearchResultTitle {
    /// List of titles of a videa
    pub runs: VecDeque<YoutubeSearchResultTitleRun>,
}

/// Single title of a video
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSearchResultTitleRun {
    /// Single title of a video
    pub text: String,
}

/// Length of a video
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSearchResultLengthText {
    /// length of the song as string
    pub simple_text: String,
}

/// Entry of `ytInitialData` variable in the HTML source code.
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeLinkResult {
    /// `contents` of `ytInitialData` variable in the HTML source code.
    pub contents: YoutubeLinkResultContents,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeLinkResultContents {
    pub two_column_watch_next_results: YoutubeLinkResultTwoColumnWatchNextResults,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeLinkResultTwoColumnWatchNextResults {
    pub primary_contents: YoutubeLinkResultPrimaryContents,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeLinkResultPrimaryContents {
    pub selection_list_renderer: YoutubeLinkResultSelectionListRenderer,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeLinkResultSelectionListRenderer {
    pub contents: Vec<YoutubeLinkResultSelectionListRendererContents>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeLinkResultSelectionListRendererContents {
    pub item_section_renderer: Option<YoutubeLinkResultItemSectionRenderer>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeLinkResultItemSectionRenderer {
    pub contents: Vec<YoutubeLinkResultItemSectionRendererContents>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeLinkResultItemSectionRendererContents {
    pub video_renderer: Option<YoutubeLinkResultVideoRenderer>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeLinkResultVideoRenderer {
    pub title:       YoutubeLinkResultTitle,
    pub video_id:    String,
    pub length_text: YoutubeLinkResultLengthText,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeLinkResultTitle {
    pub runs: VecDeque<YoutubeLinkResultTitleRun>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeLinkResultTitleRun {
    pub text: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeLinkResultLengthText {
    pub simple_text: String,
}
