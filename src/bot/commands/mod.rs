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
mod macros;

pub mod entertainment;
#[cfg(feature = "music")]
pub mod music;
pub mod others;

type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data;
