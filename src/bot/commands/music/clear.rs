use super::super::{Context, Error};
use crate::messager;

/// Bot joins the voice channel
#[poise::command(slash_command, prefix_command, category="Music", guild_only)]
pub async fn clear(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let server = ctx.data().servers.get(&guild.id).unwrap();

    // TODO: handle poisoned mutexes as well
    let mut player = server.player.lock().await;

    if player.is_queues_empty() {
        messager::send_error(&ctx, "Queue is already empty", true).await;
    } else {
        player.clear_the_queues().await;
        messager::send_sucsess(&ctx, "Queue cleared", true).await;
    }

    Ok(())
}
