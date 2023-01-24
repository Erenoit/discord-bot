use std::{env, process};

use taplo::dom::Node;

use crate::config::defaults::{AUTO_REGISTER_COMMANDS, PREFIX, VC_AUTO_CHANGE};

#[non_exhaustive]
pub(super) struct GeneralConfig {
    token:                  String,
    prefix:                 String,
    auto_register_commands: bool,
    vc_auto_change:         bool,
}

impl GeneralConfig {
    pub fn generate(config_file: &Node) -> Self {
        let token = get_value!(config_file, String, "BOT_TOKEN", "general"=>"token", "Discord token couldn't found.");
        let prefix = get_value!(config_file, String, "BOT_PREFIX", "general"=>"prefix", PREFIX);
        let auto_register_commands = get_value!(config_file, bool, "BOT_AUTO_REGISTER_COMMANDS", "general"=>"auto_register_commands", AUTO_REGISTER_COMMANDS);
        let vc_auto_change = get_value!(config_file, bool, "BOT_VC_AUTO_CHANGE", "general"=>"vc_auto_change", VC_AUTO_CHANGE);

        Self {
            token,
            prefix,
            auto_register_commands,
            vc_auto_change,
        }
    }

    #[inline(always)]
    pub const fn token(&self) -> &String { &self.token }

    #[inline(always)]
    pub const fn prefix(&self) -> &String { &self.prefix }

    #[inline(always)]
    pub const fn auto_register_commands(&self) -> bool { self.auto_register_commands }

    #[inline(always)]
    pub const fn vc_auto_change(&self) -> bool { self.vc_auto_change }
}
