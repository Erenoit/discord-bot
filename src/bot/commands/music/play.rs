use super::super::{Context, Error};
use crate::messager;

/// Adds song to queue 
#[poise::command(slash_command, prefix_command, aliases("p"), category="Music", guild_only)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Song name or Song URL"] song: String
) -> Result<(), Error> {
    let guild = ctx.guild().expect("Guild should be Some");
    let server = ctx.data().servers.get(&guild.id).unwrap();

    if song.starts_with("http://") || song.starts_with("https://") {
        // TODO: handle poisoned mutexes as well
        server.player.lock().await.play(&ctx, song).await;
    } else {
        messager::send_error(&ctx, "Search support is not ready yet! :P", false).await;
    }

    Ok(())
}
