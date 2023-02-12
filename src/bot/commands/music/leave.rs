use crate::{
    bot::commands::{Context, Error},
    get_config,
};

/// Leaves the voice channel
#[poise::command(
    slash_command,
    prefix_command,
    aliases("l"),
    category = "Music",
    guild_only
)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let servers = get_config().servers().read().await;
    let server = servers.get(&guild.id).unwrap();

    server.player.leave_voice_channel(&ctx).await;
    message!(success, ctx, ("Left the voice channel"); true);

    Ok(())
}
