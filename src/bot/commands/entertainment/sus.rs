use crate::bot::commands::{Context, Error};

/// Sends sus dog.
#[poise::command(slash_command, prefix_command, category = "Entertainment")]
pub async fn sus(ctx: Context<'_>) -> Result<(), Error> {
    message!(
        file,
        ctx,
        "IT'S SUS!",
        std::path::Path::new("./images/imposter_dog.jpg"),
        false
    );

    Ok(())
}
