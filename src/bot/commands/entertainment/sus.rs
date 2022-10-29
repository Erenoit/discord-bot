use super::super::{Context, Error};
use crate::messager;

/// Sends sus dog.
#[poise::command(slash_command, prefix_command, category="Entertainment")]
pub async fn sus(ctx: Context<'_>) -> Result<(), Error> {
    let p = std::path::Path::new("./images/imposter_dog.jpg");
    messager::send_files(&ctx, "IT'S SUS!", vec![&p], false).await;

    Ok(())
}
