use anyhow::Result;
use discord_bot::{init_config, Bot, Tui};

#[tokio::main]
async fn main() -> Result<()> {
    init_config()?;

    let bot_thread = tokio::spawn(async move {
        let mut bot = Bot::new();
        bot.run().await;
    });

    let tui_thread = tokio::spawn(async move {
        let mut tui = Tui::new().unwrap();
        tui.run().ok();
        tui.clear().ok();
    });

    bot_thread.await?;
    tui_thread.await?;

    Ok(())
}
