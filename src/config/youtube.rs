use crate::{config::defaults::{YT_AGE_RESTRICTED, YT_SEARCH_COUNT}, logger};
use std::{env, process};
use taplo::dom::Node;

// TODO add more options
// TODO: implement this configs
pub(super) struct YouTubeConfig {
    search_count: u8,
    age_restricted: bool
}

impl YouTubeConfig {
    pub fn generate(config_file: &Node) -> Self {
        let search_count = get_value!(config_file, u8, "BOT_YT_SEARCH_COUNT", "youtube"=>"search_count", YT_SEARCH_COUNT);
        let age_restricted = get_value!(config_file, bool, "BOT_YT_AGE_RESTRICTED", "youtube"=>"age_restricted", YT_AGE_RESTRICTED);

        Self { search_count, age_restricted }
    }

    #[inline(always)]
    pub const fn search_count(&self) -> u8 {
        self.search_count
    }

    #[inline(always)]
    pub const fn age_restricted(&self) -> bool {
        self.age_restricted
    }
}
