use crate::{get_config, bot::commands::{Context, Error}};

/// Skips the current playing song
#[poise::command(slash_command, prefix_command, aliases("s"), category="Music", guild_only)]
pub async fn skip(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let servers = get_config().servers().read().await;
    let server = servers.get(&guild.id).unwrap();

    // TODO: add chack for already stopped bot
    server.player.skip_song().await;

    Ok(())
}
