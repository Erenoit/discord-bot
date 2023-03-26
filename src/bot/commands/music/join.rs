use serenity::model::channel::GuildChannel;

use crate::bot::commands::{Context, Error};

/// Joins to the voice channel
#[poise::command(
    slash_command,
    prefix_command,
    aliases("j"),
    category = "Music",
    guild_only
)]
pub async fn join(
    ctx: Context<'_>,
    #[description = "Which channel to join"]
    #[channel_types("Voice")]
    channel: Option<GuildChannel>,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let servers = get_config!().servers().read().await;
    let server = servers.get(&guild.id).unwrap();

    let channel_id = if let Some(channel) = channel {
        channel.id
    } else if let Some(channel_id) = guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id)
    {
        channel_id
    } else {
        message!(error, ctx, ("Couldn't connect to a voice channel. Neither you are in a voice channel nor you provided a channel to join."); true);
        return Ok(());
    };

    // TODO: Already joined. Would you like to change?
    server.player.connect_to_voice_channel(&channel_id).await;

    message!(success, ctx, ("Connected to the voice channel"); true);

    Ok(())
}
