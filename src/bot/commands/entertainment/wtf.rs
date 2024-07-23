use crate::bot::commands::{Context, Error};

/// Sends sus dog.
#[poise::command(slash_command, prefix_command, category = "Entertainment")]
pub async fn wtf(ctx: Context<'_>) -> Result<(), Error> {
    message!(
        file,
        ctx,
        "I JUDGE YOU!",
        std::path::Path::new("./images/crow_of_judgement.jpg"),
        false
    );

    Ok(())
}
