use super::config::Config;
use crate::logger;

use std::sync::Arc;
use colored::Colorize;
use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready,
    },
    prelude::{
        Context,
        EventHandler,
    },
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
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot
        || msg.guild_id.is_none()
        || !msg.content.starts_with(self.config.prefix()) {
            return;
        }

        let trim_msg: Vec<_> = msg.content[self.config.prefix().len()..]
        .trim()
        .split(" ")
        .collect();

        //if trim_msg[0] == "ping" {
        //    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        //        logger::log(format!("Error sending message: {:?}", why), None);
        //    }
        //}
    }

    async fn ready(&self, _: Context, ready: Ready) {
        logger::log(format!("{} is online!", ready.user.name.magenta()), None);
    }
}
