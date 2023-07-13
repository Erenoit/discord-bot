//! This is the main file of the project.

// Clippy lints (last check 1.70.0)
#![warn(clippy::cargo)]
#![allow(clippy::cargo_common_metadata)] // Not going tp release on crates.io
#![warn(clippy::complexity)]
#![deny(clippy::correctness)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![allow(clippy::unreadable_literal)] // Only used for colors
#![warn(clippy::perf)]
#![allow(clippy::restriction)] // Enabling everything is not recomanded
#![warn(clippy::allow_attributes_without_reason)] // Not stable yet // force
#![warn(clippy::allow_attributes)] // Not stable yet // force
#![warn(clippy::as_underscore)] // forbid
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::deref_by_slicing)]
#![warn(clippy::dbg_macro)] // forbid
#![warn(clippy::disallowed_script_idents)]
#![warn(clippy::empty_drop)] // forbid
#![warn(clippy::empty_structs_with_brackets)]
#![warn(clippy::exhaustive_structs)]
#![warn(clippy::exit)]
// #![warn(clippy::expect_used)]
#![warn(clippy::format_push_string)]
#![warn(clippy::fn_to_numeric_cast_any)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::impl_trait_in_params)] // forbid
#![warn(clippy::large_include_file)]
#![warn(clippy::let_underscore_must_use)]
#![warn(clippy::lossy_float_literal)]
#![warn(clippy::map_err_ignore)] // forbid
#![warn(clippy::map_err_ignore)] // forget
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::mixed_read_write_in_expression)]
#![warn(clippy::multiple_inherent_impl)]
#![warn(clippy::mutex_atomic)]
#![warn(clippy::rc_mutex)] // forbid
#![warn(clippy::rest_pat_in_fully_bound_structs)]
#![warn(clippy::same_name_method)]
#![warn(clippy::self_named_module_files)]
#![warn(clippy::str_to_string)] // forbid
#![warn(clippy::string_to_string)] // forbid
#![warn(clippy::tests_outside_test_module)] // forbid
#![warn(clippy::undocumented_unsafe_blocks)] // forbid
#![warn(clippy::unnecessary_self_imports)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(clippy::unseparated_literal_suffix)] // forbid
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
