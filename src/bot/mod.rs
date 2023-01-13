mod commands;
mod event;

pub use commands::Context;

use event::Handler;
use crate::get_config;
use serenity::prelude::GatewayIntents;
use songbird::SerenityInit;

pub struct Bot;

impl Bot {
    pub fn new() -> Self {
        Self
    }

    pub async fn run(&mut self) {
        let options = poise::FrameworkOptions {
            commands: vec![
                commands::others::help::help(),
                commands::others::register::register(),
                commands::others::ping::ping(),
                commands::entertainment::sus::sus(),
                commands::entertainment::meme::meme(),
                commands::music::join::join(),
                commands::music::leave::leave(),
                commands::music::play::play(),
                commands::music::stop::stop(),
                commands::music::skip::skip(),
                commands::music::repeat::repeat(),
                commands::music::music::music(),
                commands::music::queue::queue(),
                commands::music::clear::clear(),
                commands::music::shuffle::shuffle(),
            ],

            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(get_config().prefix().to_string()),
                mention_as_prefix: false,
                execute_self_messages: false,
                ignore_bots: true,

                ..Default::default()
            },

            ..Default::default()
        };

        poise::Framework::builder()
        .token(get_config().token())
        .intents(GatewayIntents::all())
        .options(options)
        .client_settings(move |c| {
            c.event_handler(Handler::new())
                .register_songbird_with(get_config().songbird())
        })
        .setup(|_ctx, _data_about_bot, _framework| {
            Box::pin(async move {
                Ok(commands::Data)
            })
        })
        .run_autosharded()
        .await
        .expect("Client error");
    }
}

impl Default for Bot {
    fn default() -> Self {
        Self::new()
    }
}
