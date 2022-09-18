use crate::{
    bot::commands::Context,
    logger,
};
use std::sync::Arc;
use songbird::Call;
use serenity::{model::id::{ChannelId, GuildId}, prelude::Mutex};

#[inline(always)]
pub async fn join(ctx: &Context<'_>, guild_id: &GuildId, channel_id: &ChannelId) -> Arc<Mutex<Call>> {
    let manager = songbird::get(ctx.discord()).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let (call_mutex, result) = manager.join(*guild_id, *channel_id).await;

    if let Err(why) = result {
        logger::error("Couldn't join the voice channel.".to_string(), Some(why.to_string()));
    } else {
        let mut call = call_mutex.lock().await;
        if let Err(why) = call.deafen(true).await {
            logger::error("Couldn't deafen the bot.".to_string(), Some(why.to_string()));
        }
    }

    call_mutex
}

