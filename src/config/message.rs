//! Message Configuration.

use anyhow::Result;
#[cfg(feature = "config_file")]
use taplo::dom::Node;

use crate::config::defaults::{
    ALWAYS_EMBED,
    ERROR_COLOR,
    INTERACTION_TIME_LIMIT,
    NORMAL_COLOR,
    RANDOM_EMBED_COLORS,
    SUCCESS_COLOR,
};
#[cfg(not(feature = "config_file"))]
use crate::config::Node;

/// Message configuration.
#[non_exhaustive]
pub(super) struct MessageConfig {
    /// Whether to always use embed messages.
    always_embed:           bool,
    /// Whether to use random colors for embed messages.
    random_embed_colors:    bool,
    /// Color to use in embed message when the given command successes.
    success_color:          u32,
    /// Color to use in embed message when there is no success/error.
    normal_color:           u32,
    /// Color to use in embed message when the given command errors.
    error_color:            u32,
    /// Time limit for interaction messages.
    interaction_time_limit: u64,
}

impl MessageConfig {
    /// Generate a new `MessageConfig` from the config file.
    pub fn generate(config_file: &Node) -> Result<Self> {
        let always_embed = get_value!(config_file, bool, "BOT_MSG_ALWAYS_EMBED", "message"=>"always_embed", ALWAYS_EMBED)?;
        let random_embed_colors = get_value!(config_file, bool, "BOT_MSG_RANDOM_EMBED_COLORS", "message"=>"random_embed_colors", RANDOM_EMBED_COLORS)?;
        let success_color = get_value!(config_file, u32, "BOT_MSG_ALWAYS_EMBED", "message"=>"always_embed", SUCCESS_COLOR)?;
        let normal_color = get_value!(config_file, u32, "BOT_MSG_ALWAYS_EMBED", "message"=>"always_embed", NORMAL_COLOR)?;
        let error_color = get_value!(config_file, u32, "BOT_MSG_ALWAYS_EMBED", "message"=>"always_embed", ERROR_COLOR)?;
        let interaction_time_limit = get_value!(config_file, u64, "BOT_MSG_ALWAYS_EMBED", "message"=>"always_embed", INTERACTION_TIME_LIMIT)?;

        Ok(Self {
            always_embed,
            random_embed_colors,
            success_color,
            normal_color,
            error_color,
            interaction_time_limit,
        })
    }

    /// Returns whether to always use embed messages.
    pub const fn always_embed(&self) -> bool { self.always_embed }

    /// Returns whether to use random colors for embed messages.
    pub const fn random_embed_colors(&self) -> bool { self.random_embed_colors }

    /// Returns the color to use in embed message when the given command
    /// successes.
    pub const fn success_color(&self) -> u32 { self.success_color }

    /// Returns the color to use in embed message when there is no
    /// success/error.
    pub const fn normal_color(&self) -> u32 { self.normal_color }

    /// Returns the color to use in embed message when the given command errors.
    pub const fn error_color(&self) -> u32 { self.error_color }

    /// Returns the time limit for interaction messages.
    pub const fn interaction_time_limit(&self) -> u64 { self.interaction_time_limit }
}
