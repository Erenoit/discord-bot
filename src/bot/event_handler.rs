use super::config::Config;
use crate::logger;

use std::sync::Arc;
use colored::Colorize;
use serenity::{
    async_trait,
    model::gateway::Ready,
    client::{EventHandler, Context},
};

pub(super) struct Handler {
    config: Arc<Config>
}

impl Handler {
    pub fn new(config: Arc<Config>) -> Self {
        Self{ config }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        logger::info(format!("{} is online!", ready.user.name.magenta()));
    }
}
