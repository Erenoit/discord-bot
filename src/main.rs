mod bot;
mod logger;

use bot::Bot;

use serenity::prelude::GatewayIntents;

#[tokio::main]
async fn main() {
    let intents = GatewayIntents::all();

    let mut bot = Bot::new(intents).await;
    bot.run().await;
}
