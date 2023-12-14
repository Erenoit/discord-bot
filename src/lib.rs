//! This is the main file of the project.

#![feature(iter_array_chunks)]
#![feature(let_chains)]

#[macro_use]
#[allow(unused_macros)]
mod logger;
#[macro_use]
#[allow(unused_macros)]
mod messager;

use anyhow::{anyhow, Result};
use config::Config;
use tokio::sync::OnceCell;

pub use crate::bot::Bot;

/// This variable keeps everything about discord bot.
///
/// Do not use it as is. Instead use the `init_config()` and/or `get_config()`.
static CONFIG: OnceCell<Config> = OnceCell::const_new();

/// Generate `Config` struct and pu it in `tokio::sync::OnceCell`.
///
/// This should be called *before* doing any other things.
///
/// # Errors
///
/// This function cannot fail in normal circumstances. There are two
/// possibilities:
///
/// 1. Config has errors
/// 2. Config is already initialized
pub fn init_config() -> Result<()> {
    CONFIG.set(Config::generate()?).map_or_else(
        |_| Err(anyhow!("Couldn't set the config in OnceCell")),
        |_| Ok(()),
    )
}

/// Dummy module.
///
/// I cannot find any way to use `#[macro_use]`
#[macro_use]
mod dummy_module {
    /// Get `Config` from `tokio::sync::once_cell`.
    ///
    /// Because it is generated at the start this whole error handling is
    /// unnecessary. So, expect can be use without need to check.
    ///
    /// # Panics
    /// Panics if it is called before initializating the config.
    macro_rules! get_config {
        () => {{
            use crate::CONFIG;

            CONFIG.get().expect("CONFIG should be initialized at start")
        }};
    }
}

mod bot;
mod config;
#[cfg(feature = "database")]
mod database_tables;
#[cfg(feature = "music")]
mod player;
mod server;
