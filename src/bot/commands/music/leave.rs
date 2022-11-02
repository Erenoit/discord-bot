use super::super::{Context, Error};
use crate::{CONFIG, messager};

/// Leaves the voice channel
#[poise::command(slash_command, prefix_command, aliases("l"), category="Music", guild_only)]
pub async fn leave(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let servers = CONFIG.get().unwrap().servers().read().await;
    let server = servers.get(&guild.id).unwrap();

    server.player.leave_voice_channel(&ctx).await;
    messager::send_sucsess(&ctx, "Left the voice channel", true).await;

    Ok(())
}
