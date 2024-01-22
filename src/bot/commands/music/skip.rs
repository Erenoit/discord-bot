use crate::bot::commands::{Context, Error};

/// Skips the current playing song
#[poise::command(
    slash_command,
    prefix_command,
    aliases("s"),
    category = "Music",
    guild_only
)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let server = get_server!(ctx);

    // TODO: add chack for already stopped bot
    server.player.skip_song().await;

    Ok(())
}
