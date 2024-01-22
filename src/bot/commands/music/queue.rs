use crate::bot::commands::{Context, Error};

/// Prints song queue
#[poise::command(
    slash_command,
    prefix_command,
    aliases("q"),
    category = "Music",
    guild_only
)]
pub async fn queue(ctx: Context<'_>) -> Result<(), Error> {
    let server = get_server!(ctx);

    if server.player.is_queues_empty().await {
        message!(error, ctx, ("Queue is empty"); true);
    } else {
        server.player.print_queue(&ctx).await;
    }

    Ok(())
}
