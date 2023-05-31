#![warn(clippy::cargo)]
#![allow(clippy::cargo_common_metadata)] // Not going tp release on crates.io
#![warn(clippy::complexity)]
#![deny(clippy::correctness)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)] // Not documenting
#![allow(clippy::missing_panics_doc)] // Not documenting
#![allow(clippy::must_use_candidate)] // No idea what it means
#![allow(clippy::unreadable_literal)] // Only used for colors
#![warn(clippy::perf)]
#![allow(clippy::restriction)] // Enabling everything is not recomanded
//#![warn(clippy::allow_attributes_without_reason)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::deref_by_slicing)]
#![warn(clippy::disallowed_script_idents)]
#![warn(clippy::empty_structs_with_brackets)]
#![warn(clippy::exhaustive_structs)]
#![warn(clippy::exit)]
#![warn(clippy::format_push_string)]
#![warn(clippy::if_then_some_else_none)]
//#![warn(clippy::implicit_return)]
#![warn(clippy::lossy_float_literal)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
#![warn(clippy::self_named_module_files)]
#![warn(clippy::str_to_string)]
#![warn(clippy::string_to_string)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(clippy::unwrap_in_result)]
//#![warn(clippy::unwrap_used)]
#![warn(clippy::style)]
#![warn(clippy::suspicious)]
#![feature(iter_array_chunks)]

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
