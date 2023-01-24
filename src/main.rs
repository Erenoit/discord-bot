use anyhow::Result;
use discord_bot::{init_config, Bot};

#[tokio::main]
async fn main() -> Result<()> {
    init_config()?;

    let mut bot = Bot::new();
    bot.run().await;

    Ok(())
}
