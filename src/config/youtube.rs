//! `YouTube` Configuration.

use anyhow::Result;
#[cfg(feature = "config_file")]
use taplo::dom::Node;

use crate::config::defaults::{YT_AGE_RESTRICTED, YT_SEARCH_COUNT};
#[cfg(not(feature = "config_file"))]
use crate::config::Node;

// TODO add more options
// TODO: implement this configs
#[non_exhaustive]
pub(super) struct YouTubeConfig {
    /// Number of results to show when searching for a song.
    search_count:   u8,
    /// Whether to allow age restricted videos.
    age_restricted: bool,
}

impl YouTubeConfig {
    /// Generate a new `YouTubeConfig` from the config file.
    pub fn generate(config_file: &Node) -> Result<Self> {
        let search_count = get_value!(config_file, u8, "BOT_YT_SEARCH_COUNT", "youtube"=>"search_count", YT_SEARCH_COUNT)?;
        let age_restricted = get_value!(config_file, bool, "BOT_YT_AGE_RESTRICTED", "youtube"=>"age_restricted", YT_AGE_RESTRICTED)?;

        Ok(Self { search_count, age_restricted })
    }

    /// Returns the number of results to show when searching for a song.
    pub const fn search_count(&self) -> u8 { self.search_count }

    /// Returns whether to allow age restricted videos.
    pub const fn age_restricted(&self) -> bool { self.age_restricted }
}
