use discord_bot::{
    bot::Bot,
    config::Config,
    CONFIG,
};


#[tokio::main]
async fn main() {
    if let Err(why) = CONFIG.set(Config::generate()) {
        panic!("Config could not be created: {}", why);
    }

    let mut bot = Bot::new();
    bot.run().await;
}
