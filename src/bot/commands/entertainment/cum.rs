use crate::bot::commands::{Context, Error};

const CUM_THE_CUM_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/images/cum_the_cum.gif",
));

/// Sends cum the cum
#[poise::command(slash_command, prefix_command, category = "Entertainment")]
pub async fn cum(ctx: Context<'_>) -> Result<(), Error> {
    message!(
        file,
        bytes,
        ctx,
        "I CAME!",
        CUM_THE_CUM_BYTES;
        "cum_the_cum.gif",
        false
    );

    Ok(())
}
