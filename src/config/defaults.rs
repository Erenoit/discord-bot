//! Default values for the config.
//!
//! This file contains the default values for the config. These values are used
//! when the config file is not present or when a value is missing from the
//! config file.

/// Default Prefix
pub(super) const PREFIX: &str = "-";
/// Default Auto Register Commands
pub(super) const AUTO_REGISTER_COMMANDS: bool = true;
/// Default VC Auto Change
#[cfg(feature = "music")]
pub(super) const VC_AUTO_CHANGE: bool = false;
/// Default Enable Spotify
#[cfg(feature = "spotify")]
pub(super) const ENABLE_SPOTIFY: bool = false;
/// Default YT Search Count
#[cfg(feature = "music")]
pub(super) const YT_SEARCH_COUNT: u8 = 5;
/// Default YT Age Restricted
#[cfg(feature = "music")]
pub(super) const YT_AGE_RESTRICTED: bool = false;
/// Default Enable Database
#[cfg(feature = "database")]
pub(super) const ENABLE_DATABASE: bool = true;
/// Default Always Embed
pub(super) const ALWAYS_EMBED: bool = false;
/// Default Random Embed Colors
pub(super) const RANDOM_EMBED_COLORS: bool = false;
/// Default Embed Success Color
pub(super) const SUCCESS_COLOR: u32 = 0x00FF00;
/// Default Embed Normal Color
pub(super) const NORMAL_COLOR: u32 = 0x0000FF;
/// Default Embed Error Color
pub(super) const ERROR_COLOR: u32 = 0xFF0000;
/// Default Interaction Time Limit
pub(super) const INTERACTION_TIME_LIMIT: u64 = 30;
