use super::super::{Context, Error};
use crate::messager;

/// Prints song queue
#[poise::command(slash_command, prefix_command, aliases("q"), category="Music", guild_only)]
pub async fn queue(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let server = ctx.data().servers.get(&guild.id).unwrap();

    // TODO: handle poisoned mutexes as well
    let player = server.player.lock().await;

    if player.is_queues_empty() {
        messager::send_error(&ctx, "Queue is empty", true).await;
    } else {
        player.print_queue(&ctx).await;
    }

    Ok(())
}
