use std::env;

use anyhow::Result;
use taplo::dom::Node;

use crate::config::defaults::{
    ALWAYS_EMBED,
    ERROR_COLOR,
    INTERACTION_TIME_LIMIT,
    NORMAL_COLOR,
    RANDOM_EMBED_COLORS,
    SUCCESS_COLOR,
};

#[non_exhaustive]
pub(super) struct MessageConfig {
    always_embed:           bool,
    random_embed_colors:    bool,
    success_color:          u32,
    normal_color:           u32,
    error_color:            u32,
    interaction_time_limit: u64,
}

impl MessageConfig {
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

    pub const fn always_embed(&self) -> bool { self.always_embed }

    pub const fn random_embed_colors(&self) -> bool { self.random_embed_colors }

    pub const fn success_color(&self) -> u32 { self.success_color }

    pub const fn normal_color(&self) -> u32 { self.normal_color }

    pub const fn error_color(&self) -> u32 { self.error_color }

    pub const fn interaction_time_limit(&self) -> u64 { self.interaction_time_limit }
}
