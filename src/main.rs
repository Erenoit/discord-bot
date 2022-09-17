mod bot;
mod logger;

use bot::Bot;

#[tokio::main]
async fn main() {
    let mut bot = Bot::new();
    bot.run().await;
}
