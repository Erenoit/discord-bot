#![warn(clippy::cargo)]
    #![allow(clippy::cargo_common_metadata)]
#![warn(clippy::complexity)]
#![deny(clippy::correctness)]
#![warn(clippy::perf)]
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
        panic!("Config could not be created: {}", why);
    }
}

#[inline(always)]
fn get_config() -> &'static Config {
    CONFIG.get().expect("CONFIG should be initialized at start")
}

