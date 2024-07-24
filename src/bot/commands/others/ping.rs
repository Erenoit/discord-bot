use crate::bot::commands::{Context, Error};

/// Check if bot is online
#[poise::command(slash_command, prefix_command, category = "Others")]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    message!(normal, ctx, ("Online"); ("Pong! :stuck_out_tongue_winking_eye:"); true);

    Ok(())
}
