use super::super::{Context, Error};
use crate::CONFIG;

/// Skips the current playing song
#[poise::command(slash_command, prefix_command, aliases("s"), category="Music", guild_only)]
pub async fn skip(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let servers = CONFIG.get().unwrap().servers().read().await;
    let server = servers.get(&guild.id).unwrap();

    // TODO: handle poisoned mutexes as well
    // TODO: add chack for already stopped bot
    server.player.lock().await.skip_song().await;

    Ok(())
}
