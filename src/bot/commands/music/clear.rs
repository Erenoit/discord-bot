use crate::bot::commands::{Context, Error};

/// Clears the queue but do not stop current playing song
#[poise::command(slash_command, prefix_command, category = "Music", guild_only)]
pub async fn clear(ctx: Context<'_>) -> Result<(), Error> {
    let (_guild, server) = get_common!(ctx);

    if server.player.is_queues_empty().await {
        message!(error, ctx, ("Queue is already empty"); true);
    } else {
        server.player.clear_the_queues().await;
        message!(success, ctx, ("Queue cleared"); true);
    }

    Ok(())
}
