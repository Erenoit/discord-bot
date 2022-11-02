use super::super::{Context, Error};
use crate::{CONFIG, messager};

/// Prints song queue
#[poise::command(slash_command, prefix_command, aliases("q"), category="Music", guild_only)]
pub async fn queue(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let servers = CONFIG.get().unwrap().servers().read().await;
    let server = servers.get(&guild.id).unwrap();

    if server.player.is_queues_empty().await {
        messager::send_error(&ctx, "Queue is empty", true).await;
    } else {
        server.player.print_queue(&ctx).await;
    }

    Ok(())
}
