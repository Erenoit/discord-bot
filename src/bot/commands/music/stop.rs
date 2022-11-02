use super::super::{Context, Error};
use crate::messager;

/// Stops the song stream and clears the queue
#[poise::command(slash_command, prefix_command, aliases("st"), category="Music", guild_only)]
pub async fn stop(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let server = ctx.data().servers.get(&guild.id).unwrap();

    // TODO: handle poisoned mutexes as well
    // TODO: add chack for already stopped bot
    server.player.lock().await.clear_the_queues().await;
    server.player.lock().await.stop_stream(&ctx).await;
    messager::send_sucsess(&ctx, ":sob:", true).await;

    Ok(())
}
