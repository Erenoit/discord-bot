//! Yu-ouTube struct to scrape search result
//!
//! It may have other struct to scrape other things in the future.

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

/// Entry of `ytInitialData` variable in the HTML source code.
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSearch {
    /// `contents` of `ytInitialData` variable in the HTML source code.
    pub contents: YoutubeContentsSearch,
}

/// Entry of `ytInitialData` variable in the HTML source code.
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeLink {
    /// `contents` of `ytInitialData` variable in the HTML source code.
    pub contents: YoutubeContentsWatch,
}

/// Container
///
/// Only difference from `YoutubeContentsSearch` is the field name
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeContentsWatch {
    /// Container
    pub two_column_watch_next_results: YoutubeTwoColumnSelection,
}

/// Container
///
/// Only difference from `YoutubeContentsWatch` is the field name
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeContentsSearch {
    /// Container
    pub two_column_search_results_renderer: YoutubeTwoColumnSection,
}

/// Container
///
/// Only difference from `YoutubeTwoColumnSection` is the type of
/// `primary_contents` Yes, it was way aesier to create neew type instead of
/// using generics because [`eserialize`](serde::Deserilize) has a lifetime.
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeTwoColumnSelection {
    /// Container
    pub primary_contents: YoutubePrimaryContentsSelection,
}

/// Container
///
/// Only difference from `YoutubeTwoColumnSelection` is the type of
/// `primary_contents` Yes, it was way aesier to create neew type instead of
/// using generics because [`eserialize`](serde::Deserilize) has a lifetime.
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeTwoColumnSection {
    /// Container
    pub primary_contents: YoutubePrimaryContentsSection,
}
/// Container
///
/// Only difference from `YoutubePrimaryContentsSection` is the field name
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubePrimaryContentsSelection {
    /// Container
    pub selection_list_renderer: YoutubeSectionListRenderer,
}

/// Container
///
/// Only difference from `YoutubePrimaryContentsSelection` is the field name
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubePrimaryContentsSection {
    /// Container
    pub section_list_renderer: YoutubeSectionListRenderer,
}

/// Container
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSectionListRenderer {
    /// Container
    pub contents: Vec<YoutubeSectionListRendererContents>,
}

/// Container
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeSectionListRendererContents {
    /// Container
    pub item_section_renderer: Option<YoutubeItemSectionRenderer>,
}

/// A struct that holds list of videos
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeItemSectionRenderer {
    /// A list of videos
    pub contents: Vec<YoutubeItemSectionRendererContents>,
}

/// Struct that hold a single video
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeItemSectionRendererContents {
    /// A single video
    pub video_renderer: Option<YoutubeVideoRenderer>,
}

/// Information about a video
#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
pub struct YoutubeVideoRenderer {
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
