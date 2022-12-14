use crate::{get_config, bot::commands::{Context, Error}, messager, player::{context_to_voice_channel_id, Song}};

/// Adds song to queue 
#[poise::command(slash_command, prefix_command, aliases("p"), category="Music", guild_only)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Song name or Song URL"] song: String
) -> Result<(), Error> {
    let guild = ctx.guild().expect("Guild should be Some");
    let servers = get_config().servers().read().await;
    let server = servers.get(&guild.id).unwrap();

    if !server.player.is_in_vc().await {
        if let Some(channel_id) = context_to_voice_channel_id(&ctx) {
            server.player.connect_to_voice_channel(&channel_id).await;
        } else {
            messager::send_error(&ctx, "You are not in the voice channel", true).await;
            return Ok(());
        }
    }

    let mut songs = Song::new(&ctx, song).await?;
    match songs.len() {
        0 => {
            messager::send_error(&ctx, "An error happened please try again later", false).await;
            return Ok(());
        }
        1 => messager::send_sucsess(&ctx, format!("{} has been added to the queue.", messager::bold(songs[0].title())), false).await,
        _ => messager::send_sucsess(&ctx, format!("{} songs added to the queue.", messager::bold(songs.len())), false).await
    }
    server.player.play(&mut songs).await;

    Ok(())
}
