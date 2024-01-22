use crate::bot::commands::{Context, Error};

/// Leaves the voice channel
#[poise::command(
    slash_command,
    prefix_command,
    aliases("l"),
    category = "Music",
    guild_only
)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let server = get_server!(ctx);

    server.player.leave_voice_channel(&ctx).await;
    message!(success, ctx, ("Left the voice channel"); true);

    Ok(())
}
