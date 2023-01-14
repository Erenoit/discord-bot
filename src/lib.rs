#![warn(clippy::cargo)]
    #![allow(clippy::cargo_common_metadata)] // Not going tp release on crates.io
#![warn(clippy::complexity)]
#![deny(clippy::correctness)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
    #![allow(clippy::inline_always)]       // Should learn more about inline
    #![allow(clippy::let_underscore_drop)] // Not understand why shouldn't I drop immediately
    #![allow(clippy::missing_panics_doc)]  // Not documenting
    #![allow(clippy::must_use_candidate)]  // No idea what it means
    #![allow(clippy::unreadable_literal)]  // Only used for colors
#![warn(clippy::perf)]
#![allow(clippy::restriction)] // Enabling everything is not recomanded
    //#![warn(clippy::allow_attributes_without_reason)]
    #![warn(clippy::clone_on_ref_ptr)]
    #![warn(clippy::deref_by_slicing)]
    #![warn(clippy::disallowed_script_idents)]
    #![warn(clippy::empty_structs_with_brackets)]
    #![warn(clippy::exhaustive_structs)]
    //#![warn(clippy::exit)]
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

mod bot;
mod config;
mod player;
mod server;
#[allow(dead_code)]
pub mod logger;
#[allow(dead_code)]
pub mod messager;

pub use bot::Bot;

use config::Config;
use tokio::sync::OnceCell;

pub static CONFIG: OnceCell<Config> = OnceCell::const_new();

#[inline(always)]
pub fn init_config() {
    if let Err(why) = CONFIG.set(Config::generate()) {
        panic!("Config could not be created: {why}");
    }
}

#[inline(always)]
fn get_config() -> &'static Config {
    CONFIG.get().expect("CONFIG should be initialized at start")
}

