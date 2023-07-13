//! General Configuration

use anyhow::Result;
#[cfg(feature = "config_file")]
use taplo::dom::Node;

#[cfg(feature = "music")]
use crate::config::defaults::VC_AUTO_CHANGE;
use crate::config::defaults::{AUTO_REGISTER_COMMANDS, PREFIX};
#[cfg(not(feature = "config_file"))]
use crate::config::Node;

/// General configuration.
#[non_exhaustive]
pub(super) struct GeneralConfig {
    /// Discord token.
    token:                  String,
    /// Prefix for commands.
    prefix:                 String,
    /// Whether to auto register commands.
    auto_register_commands: bool,
    /// Whether to auto change voice channel when user sended the command from
    /// another voice cahnnel.
    #[cfg(feature = "music")]
    vc_auto_change:         bool,
}

impl GeneralConfig {
    /// Generate a new `GeneralConfig` from the config file.
    pub fn generate(config_file: &Node) -> Result<Self> {
        let token = get_value!(config_file, String, "BOT_TOKEN", "general"=>"token", "Discord token couldn't found.")?;
        let prefix = get_value!(config_file, String, "BOT_PREFIX", "general"=>"prefix", PREFIX)?;
        let auto_register_commands = get_value!(config_file, bool, "BOT_AUTO_REGISTER_COMMANDS", "general"=>"auto_register_commands", AUTO_REGISTER_COMMANDS)?;
        #[cfg(feature = "music")]
        let vc_auto_change = get_value!(config_file, bool, "BOT_VC_AUTO_CHANGE", "general"=>"vc_auto_change", VC_AUTO_CHANGE)?;

        Ok(Self {
            token,
            prefix,
            auto_register_commands,
            #[cfg(feature = "music")]
            vc_auto_change,
        })
    }

    /// Returns the Discord token.
    pub const fn token(&self) -> &String { &self.token }

    /// Returns the prefix for commands.
    pub const fn prefix(&self) -> &String { &self.prefix }

    /// Returns whether to auto register commands.
    pub const fn auto_register_commands(&self) -> bool { self.auto_register_commands }

    /// Returns whether to auto change voice channel when user sended the
    /// command from another voice cahnnel.
    #[cfg(feature = "music")]
    pub const fn vc_auto_change(&self) -> bool { self.vc_auto_change }
}
