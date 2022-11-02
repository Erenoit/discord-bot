pub mod bot;
pub mod config;
pub mod player;
pub mod server;
#[allow(dead_code)]
pub mod logger;
#[allow(dead_code)]
pub mod messager;

use config::Config;
use tokio::sync::OnceCell;

pub static CONFIG: OnceCell<Config> = OnceCell::const_new();
