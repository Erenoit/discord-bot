mod event_handler;
mod config;

use event_handler::Handler;
use config::Config;

use std::sync::Arc;
use serenity::prelude::{
    Client,
    GatewayIntents,
};

pub struct Bot {
    client: Client,
    config: Arc<Config>,
}

impl Bot {
    pub async fn new(intents: GatewayIntents) -> Self {
        let config = Arc::new(Config::generate());

        let client = Client::builder(config.token(), intents)
            .event_handler(Handler::new(Arc::clone(&config)))
            .await
            .expect("Failed to create the client");

        Self {
            client,
            config,
        }
    }

    pub async fn run(&mut self) {
        if let Err(why) = self.client.start().await {
            println!("Client error: {:?}", why);
        }
    }
}
