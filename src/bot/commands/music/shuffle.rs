use super::super::{Context, Error};
use crate::{CONFIG, messager};

/// Shuffles the queue
#[poise::command(slash_command, prefix_command, category="Music", guild_only)]
pub async fn shuffle(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let servers = CONFIG.get().unwrap().servers().read().await;
    let server = servers.get(&guild.id).unwrap();

    if server.player.is_queues_empty().await {
        messager::send_error(&ctx, "Queue is empty", true).await;
    } else {
        // TODO: r u sure??
        server.player.shuffle_song_queue().await;
        messager::send_sucsess(&ctx, "Queue shuffled", true).await;
    }

    Ok(())
}
