use crate::{get_config, messager, bot::commands::{Context, Error}};

/// Shuffles the queue
#[poise::command(slash_command, prefix_command, category="Music", guild_only)]
pub async fn shuffle(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let servers = get_config().servers().read().await;
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
