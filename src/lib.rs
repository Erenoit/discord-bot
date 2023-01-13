#![warn(clippy::cargo)]
    #![allow(clippy::cargo_common_metadata)] // Not going tp release on crates.io
#![warn(clippy::complexity)]
#![deny(clippy::correctness)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
    #![allow(clippy::inline_always)]       // Should learn more about inline
    #![allow(clippy::missing_panics_doc)]  // Not documenting
    #![allow(clippy::must_use_candidate)]  // No idea what it means
    #![allow(clippy::unreadable_literal)]  // Only used for colors
    #![allow(clippy::let_underscore_drop)] // Not understand why shouldn't I drop immediately
#![warn(clippy::perf)]
#![allow(clippy::restriction)]
    #![warn(clippy::clone_on_ref_ptr)]
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

