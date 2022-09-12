use super::super::{Context, Error};

/// Check if bot is online.
#[poise::command(slash_command, prefix_command, category="Others")]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let msg = "Pong! :stuck_out_tongue_winking_eye:";

    _ = ctx.send(|f| {
        f.content(msg)
            .ephemeral(true)
    })
    .await;

    Ok(())
}
