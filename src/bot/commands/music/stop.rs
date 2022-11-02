use super::super::{Context, Error};
use crate::{get_config, messager};

/// Stops the song stream and clears the queue
#[poise::command(slash_command, prefix_command, aliases("st"), category="Music", guild_only)]
pub async fn stop(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let servers = get_config().servers().read().await;
    let server = servers.get(&guild.id).unwrap();

    // TODO: add chack for already stopped bot
    server.player.clear_the_queues().await;
    server.player.stop_stream().await;
    messager::send_sucsess(&ctx, ":sob:", true).await;

    Ok(())
}
