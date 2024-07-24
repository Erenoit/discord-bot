use crate::bot::commands::{Context, Error};

const IMPOSTER_DOG: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/images/imposter_dog.jpg"
));

/// Sends sus dog
#[poise::command(slash_command, prefix_command, category = "Entertainment")]
pub async fn sus(ctx: Context<'_>) -> Result<(), Error> {
    message!(
        file,
        bytes,
        ctx,
        "IT'S SUS!",
        IMPOSTER_DOG;
        "imposter_dog.jpg",
        false
    );

    Ok(())
}
