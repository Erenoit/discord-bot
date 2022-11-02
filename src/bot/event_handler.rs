use crate::logger;

use colored::Colorize;
use serenity::{
    async_trait,
    model::gateway::Ready,
    client::{EventHandler, Context},
};

pub struct Handler;
impl Handler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        logger::info(format!("{} is online!", ready.user.name.magenta()));
    }
}
