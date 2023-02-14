use crate::{
    bot::commands::{Context, Error},
    get_config,
};

/// Prints song queue
#[poise::command(
    slash_command,
    prefix_command,
    aliases("q"),
    category = "Music",
    guild_only
)]
pub async fn queue(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let servers = get_config().servers().read().await;
    let server = servers.get(&guild.id).unwrap();

    if server.player.is_queues_empty().await {
        message!(error, ctx, ("Queue is empty"); true);
    } else {
        server.player.print_queue(&ctx).await;
    }

    Ok(())
}
