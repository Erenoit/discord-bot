#[cfg(not(feature = "shuttle"))]
use anyhow::Result;
use discord_bot::{init_config, Bot};

#[cfg(not(feature = "shuttle"))]
#[tokio::main]
async fn main() -> Result<()> {
    init_config()?;

    let mut bot = Bot::new();
    bot.run().await;

    Ok(())
}

#[cfg(feature = "shuttle")]
#[expect(clippy::unused_async, reason = "Required by Shuttle")]
#[shuttle_runtime::main]
async fn main() -> Result<Bot, shuttle_runtime::Error> {
    init_config()?;

    let bot = Bot::new();

    Ok(bot)
}
