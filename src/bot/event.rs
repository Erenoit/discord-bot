use crate::{get_config, logger, server::Server};
use colored::Colorize;
use serenity::{
    async_trait,
    model::{gateway::Ready, guild::{Guild, UnavailableGuild}},
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
        let mut servers = get_config().servers().write().await;

        for g in ready.guilds {
            logger::secondary_info(format!("{}", g.id));
            servers.insert(g.id, Server::new(g.id));
        }

        logger::info(format!("{} is online!", ready.user.name.magenta()));
    }

    async fn guild_create(&self, _ctx: Context, guild: Guild, _is_new: bool) {
        logger::info("Joined to a new server.");
        logger::secondary_info(format!("Guild id: {}", guild.id));
        get_config().servers().write().await.insert(guild.id, Server::new(guild.id));
    }

    async fn guild_delete(&self, _ctx: Context, incomplate: UnavailableGuild, _full: Option<Guild>) {
        logger::info("Removed from a server.");
        logger::secondary_info(format!("Guild id: {}", incomplate.id));
        get_config().servers().write().await.remove(&incomplate.id);
    }
}
