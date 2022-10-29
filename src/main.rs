mod bot;
mod player;
mod common;
#[allow(dead_code)]
mod logger;
#[allow(dead_code)]
mod messager;

use bot::Bot;

#[tokio::main]
async fn main() {
    let mut bot = Bot::new();
    bot.run().await;
}
