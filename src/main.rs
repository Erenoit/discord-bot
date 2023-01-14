use discord_bot::{init_config, Bot};

#[tokio::main]
async fn main() {
    init_config();

    let mut bot = Bot::new();
    bot.run().await;
}
