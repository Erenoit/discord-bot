use super::super::{Context, Error};
use crate::player::play::open_yt_url;

/// Adds song to queue 
#[poise::command(slash_command, prefix_command, category="Music", guild_only)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Song name or Song URL"] song: String
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;


    if !song.starts_with("http://") && !song.starts_with("https://") {
        // TODO: Add search support
        _ = ctx.say("Search support is not ready yet! :P").await;
    } else if song.contains("youtube.com") {
        open_yt_url(&ctx, &guild_id, &song).await;
    } else if song.contains("spotify.com") {
        // TODO: Add Spotify link support
        _ = ctx.say("Spotify url support is not ready yet! :P").await;
    } else {
        _ = ctx.say("Link is from unsupported source. :angry:").await;
    }

    Ok(())
}
