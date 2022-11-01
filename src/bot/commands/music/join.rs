use super::super::{Context, Error};
use crate::messager;
use serenity::model::channel::GuildChannel;

/// Joins to the voice channel
#[poise::command(slash_command, prefix_command, aliases("j"), category="Music", guild_only)]
pub async fn join(
    ctx: Context<'_>,
    #[description = "Which channel to join"]
    #[channel_types("Voice")]
    channel: Option<GuildChannel>,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let server = ctx.data().servers.get(&guild.id).unwrap();

    let channel_id = if let Some(channel) = channel {
        channel.id
    } else if let Some(channel_id) = guild.voice_states.get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id) {
        channel_id
    } else {
        messager::send_error(&ctx, "Couldn't connect to a voice channel. Neither you are in a voice channel nor you provided a channel to join.", true).await;
        return Ok(());
    };

    // TODO: Already joined. Would you like to change?
    // TODO: handle poisoned mutexes as well
    server.player.lock().await.connect_to_voice_channel(&ctx, &channel_id).await;

    messager::send_sucsess(&ctx, "Connected to the voice channel", true).await;

    Ok(())
}
