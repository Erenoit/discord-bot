use crate::{server::Server, CONFIG, logger};

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
        // TODO: find a better way to init servers (if there is)
        logger::info("Guilds:");
        let mut servers = CONFIG.get().unwrap().servers().write().await;

        for g in ready.guilds {
            logger::secondary_info(format!("{}", g.id));
            servers.insert(g.id, Server::new(g.id));
        }

        logger::info(format!("{} is online!", ready.user.name.magenta()));
    }
}
