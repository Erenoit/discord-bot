use crate::{messager, bot::commands::{Context, Error}};

/// Check if bot is online.
#[poise::command(slash_command, prefix_command, category="Others")]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    messager::send_normal(&ctx, "Online",  "Pong! :stuck_out_tongue_winking_eye:", true).await;

    Ok(())
}
