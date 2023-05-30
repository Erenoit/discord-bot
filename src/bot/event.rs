use std::sync::Arc;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        gateway::Ready,
        guild::{Guild, UnavailableGuild},
    },
};

use crate::server::Server;

pub struct Handler;
impl Handler {
    pub const fn new() -> Self { Self }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        // TODO: find a better way to init servers (if there is)
        log!(info, "Guilds:");
        let mut servers = get_config!().servers().write().await;

        for g in ready.guilds {
            log!(info, ; "{}", (g.id));
            servers.insert(g.id, Arc::new(Server::new(g.id)));
        }

        log!(info, "{} is online!", (ready.user.name.magenta()));
    }

    async fn guild_create(&self, _ctx: Context, guild: Guild, is_new: bool) {
        if is_new {
            log!(info, "Joined to a new server."; "Guild id: {}", (guild.id));
            get_config!()
                .servers()
                .write()
                .await
                .insert(guild.id, Arc::new(Server::new(guild.id)));
        }
    }

    async fn guild_delete(
        &self,
        _ctx: Context,
        incomplate: UnavailableGuild,
        _full: Option<Guild>,
    ) {
        log!(info, "Removed from a server."; "Guild id: {}", (incomplate.id));
        get_config!().servers().write().await.remove(&incomplate.id);
    }
}
