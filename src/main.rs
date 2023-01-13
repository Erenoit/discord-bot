use discord_bot::{
    bot::Bot,
    init_config
};


#[tokio::main]
async fn main() {
    init_config();

    let mut bot = Bot::new();
    bot.run().await;
}
