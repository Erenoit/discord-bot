use super::super::{Context, Error};
use crate::messager;

/// Shuffles the queue
#[poise::command(slash_command, prefix_command, category="Music", guild_only)]
pub async fn shuffle(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let server = ctx.data().servers.get(&guild.id).unwrap();

    // TODO: handle poisoned mutexes as well
    let mut player = server.player.lock().await;

    if player.is_queues_empty() {
        messager::send_error(&ctx, "Queue is empty", true).await;
    } else {
        // TODO: r u sure??
        player.shuffle_song_queue().await;
        messager::send_sucsess(&ctx, "Queue shuffled", true).await;
    }

    Ok(())
}
