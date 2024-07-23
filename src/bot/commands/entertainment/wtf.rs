use crate::bot::commands::{Context, Error};

const CROW_OF_JUDGEMENT_BYTES: &'static [u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/images/crow_of_judgement.jpg",
));

/// Sends crow of judgement
#[poise::command(slash_command, prefix_command, category = "Entertainment")]
pub async fn wtf(ctx: Context<'_>) -> Result<(), Error> {
    message!(
        file,
        bytes,
        ctx,
        "I JUDGE YOU!",
        CROW_OF_JUDGEMENT_BYTES;
        "crow_of_judgement.jpg",
        false
    );

    Ok(())
}
