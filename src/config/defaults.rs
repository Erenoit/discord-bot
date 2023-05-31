//! Default values for the config.
//!
//! This file contains the default values for the config. These values are used
//! when the config file is not present or when a value is missing from the
//! config file.

pub(super) const PREFIX: &str = "-";
pub(super) const AUTO_REGISTER_COMMANDS: bool = true;
#[cfg(feature = "music")]
pub(super) const VC_AUTO_CHANGE: bool = false;
#[cfg(feature = "spotify")]
pub(super) const ENABLE_SPOTIFY: bool = false;
#[cfg(feature = "music")]
pub(super) const YT_SEARCH_COUNT: u8 = 5;
#[cfg(feature = "music")]
pub(super) const YT_AGE_RESTRICTED: bool = false;
#[cfg(feature = "database")]
pub(super) const ENABLE_DATABASE: bool = true;
pub(super) const ALWAYS_EMBED: bool = false;
pub(super) const RANDOM_EMBED_COLORS: bool = false;
pub(super) const SUCCESS_COLOR: u32 = 0x00FF00;
pub(super) const NORMAL_COLOR: u32 = 0x0000FF;
pub(super) const ERROR_COLOR: u32 = 0xFF0000;
pub(super) const INTERACTION_TIME_LIMIT: u64 = 30;
