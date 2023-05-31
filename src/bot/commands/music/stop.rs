use crate::bot::commands::{Context, Error};

/// Stops the song stream and clears the queue
#[poise::command(
    slash_command,
    prefix_command,
    aliases("st"),
    category = "Music",
    guild_only
)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    let (_guild, server) = get_common!(ctx);

    // TODO: add chack for already stopped bot
    server.player.clear_the_queues().await;
    server.player.stop_stream().await;
    message!(success, ctx, (":sob:"); true);

    Ok(())
}
