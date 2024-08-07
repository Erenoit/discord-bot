//! Struct for handling `Discord` events.

use std::sync::Arc;

use colored::Colorize;
use reqwest::Client;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        gateway::Ready,
        guild::{Guild, UnavailableGuild},
    },
};
use tracing::{info, trace};

use crate::server::Server;

/// Struct for handling `Discord` events.
///
/// It currently handles `Ready`, `GuildCreate` and `GuildDelete` events.
pub struct Handler {
    reqwest_client: Client,
}

impl Handler {
    /// Creates new [`Handler`] struct.
    pub const fn new(reqwest_client: Client) -> Self { Self { reqwest_client } }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, data_about_bot: Ready) {
        // TODO: find a better way to init servers (if there is)
        let mut servers = get_config!().servers().write().await;

        for g in data_about_bot.guilds {
            trace!("Guild added: {}", g.id);
            servers.insert(
                g.id,
                Arc::new(Server::new(g.id, self.reqwest_client.clone())),
            );
        }

        info!(
            "{} is online!",
            data_about_bot.user.name.magenta()
        );
        drop(servers);
    }

    async fn guild_create(&self, _ctx: Context, guild: Guild, is_new: Option<bool>) {
        let is_new = is_new.unwrap_or(false);
        if is_new {
            trace!("Joined to a new server: {}", guild.id);
            get_config!().servers().write().await.insert(
                guild.id,
                Arc::new(Server::new(guild.id, self.reqwest_client.clone())),
            );
        }
    }

    async fn guild_delete(
        &self,
        _ctx: Context,
        incomplete: UnavailableGuild,
        _full: Option<Guild>,
    ) {
        trace!("Removed from a server: {}", incomplete.id);
        get_config!().servers().write().await.remove(&incomplete.id);
    }
}
