use super::super::{Context, Error};
use crate::{CONFIG, messager};

/// Stops the song stream and clears the queue
#[poise::command(slash_command, prefix_command, aliases("st"), category="Music", guild_only)]
pub async fn stop(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let servers = CONFIG.get().unwrap().servers().read().await;
    let server = servers.get(&guild.id).unwrap();

    // TODO: handle poisoned mutexes as well
    // TODO: add chack for already stopped bot
    server.player.lock().await.clear_the_queues().await;
    server.player.lock().await.stop_stream().await;
    messager::send_sucsess(&ctx, ":sob:", true).await;

    Ok(())
}
