use crate::bot::commands::{Context, Error};

/// Shuffles the queue
#[poise::command(slash_command, prefix_command, category = "Music", guild_only)]
pub async fn shuffle(ctx: Context<'_>) -> Result<(), Error> {
    let server = get_server!(ctx);

    if server.player.is_queues_empty().await {
        message!(error, ctx, ("Queue is empty"); true);
    } else {
        let answer = selection!(
            confirm,
            ctx,
            "You cannot unshuffle the queue. Are you sure?"
        );

        if answer {
            server.player.shuffle_song_queue().await;
            message!(success, ctx, ("Queue shuffled"); true);
        }
    }

    Ok(())
}
