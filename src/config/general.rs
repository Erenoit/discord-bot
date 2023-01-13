use crate::{config::defaults::{PREFIX, VC_AUTO_CHANGE}, logger};
use std::{env, process};
use taplo::dom::Node;

#[non_exhaustive]
pub(super) struct GeneralConfig {
    token: String,
    prefix: String,
    vc_auto_change: bool,
}

impl GeneralConfig {
    pub fn generate(config_file: &Node) -> Self {
        let token = get_value!(config_file, String, "BOT_TOKEN", "general"=>"token", "Discord token couldn't found.");
        let prefix = get_value!(config_file, String, "BOT_PREFIX", "general"=>"prefix", PREFIX);
        let vc_auto_change = get_value!(config_file, bool, "BOT_VC_AUTO_CHANGE", "general"=>"vc_auto_change", VC_AUTO_CHANGE);

        Self { token, prefix, vc_auto_change }
    }

    #[inline(always)]
    pub const fn token(&self) -> &String {
        &self.token
    }

    #[inline(always)]
    pub const fn prefix(&self) -> &String {
        &self.prefix
    }

    #[inline(always)]
    pub const fn vc_auto_change(&self) -> bool {
        self.vc_auto_change
    }
}

