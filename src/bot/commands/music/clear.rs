use super::super::{Context, Error};
use crate::{get_config, messager};

/// Clears the queue but do not stop current playing song
#[poise::command(slash_command, prefix_command, category="Music", guild_only)]
pub async fn clear(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let servers = get_config().servers().read().await;
    let server = servers.get(&guild.id).unwrap();

    if server.player.is_queues_empty().await {
        messager::send_error(&ctx, "Queue is already empty", true).await;
    } else {
        server.player.clear_the_queues().await;
        messager::send_sucsess(&ctx, "Queue cleared", true).await;
    }

    Ok(())
}
