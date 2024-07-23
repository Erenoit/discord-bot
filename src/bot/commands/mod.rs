//! The module containing all the commands for the bot.
//!
//! The commands are split into categories, which are then split into their
//! individual modules. This is done to make it easier to find commands and to
//! make it easier to add new commands.
//!
//! The categories are:
//! - Entertainment
//! - Music
//! - Others

#[macro_use]
#[allow(unused_macros)]
mod macros;

pub mod entertainment;
#[cfg(feature = "music")]
pub mod music;
pub mod others;

use reqwest::Client;

/// Error type for the bot.
type Error = Box<dyn std::error::Error + Send + Sync>;
/// Context type for the bot.
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// The data type for the bot.
pub struct Data {
    /// Common [`reqwest::Client`] for all commands. It does not need to be in
    /// [`Arc`] because it already uses it internally.
    pub reqwest_client: Client,
}
