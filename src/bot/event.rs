//! Struct for handling `Discord` events.

use std::sync::Arc;

use reqwest::Client;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        gateway::Ready,
        guild::{Guild, UnavailableGuild},
    },
};

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
        log!(info, "Guilds:");
        let mut servers = get_config!().servers().write().await;

        for g in data_about_bot.guilds {
            log!(info, ; "{}", (g.id));
            servers.insert(
                g.id,
                Arc::new(Server::new(g.id, self.reqwest_client.clone())),
            );
        }

        log!(
            info,
            "{} is online!",
            (data_about_bot.user.name.magenta())
        );
        drop(servers);
    }

    async fn guild_create(&self, _ctx: Context, guild: Guild, is_new: Option<bool>) {
        let is_new = is_new.unwrap_or(false);
        if is_new {
            log!(info, "Joined to a new server."; "Guild id: {}", (guild.id));
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
        log!(info, "Removed from a server."; "Guild id: {}", (incomplete.id));
        get_config!().servers().write().await.remove(&incomplete.id);
    }
}
