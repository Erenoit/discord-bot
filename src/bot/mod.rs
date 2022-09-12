mod commands;
mod config;
mod event_handler;

use event_handler::Handler;
use config::Config;

use std::sync::Arc;
use serenity::{
    client::ClientBuilder,
    prelude::{
        Client,
        GatewayIntents,
    },
};

pub struct Bot {
    //client: Client,
    config: Arc<Config>,
}

impl Bot {
    pub async fn new(intents: GatewayIntents) -> Self {
        let config = Arc::new(Config::generate());

        //let client = Client::builder(config.token(), intents)
        //    .event_handler(Handler::new(Arc::clone(&config)))
        //    .await
        //    .expect("Failed to create the client");

        Self {
        //    client,
            config,
        }
    }

    pub async fn run(&mut self) {
        //if let Err(why) = self.client.start().await {
        //    println!("Client error: {:?}", why);
        //}

        let options = poise::FrameworkOptions {
            commands: vec![
                commands::others::help::help(),
                commands::others::register::register(),
                commands::others::ping::ping(),
            ],
            //listener: |ctx, event, framework, user_data| {
            //    Box::pin(event_listener(ctx, event, framework, user_data))
            //},
            //on_error: |error| Box::pin(on_error(error)),
            //// Set a function to be called prior to each command execution. This
            //// provides all context of the command that would also be passed to the actual command code
            //pre_command: |ctx| Box::pin(pre_command(ctx)),
            //// Similar to `pre_command`, except will be called directly _after_
            //// command execution.
            //post_command: |ctx| Box::pin(post_command(ctx)),

            // Options specific to prefix commands, i.e. commands invoked via chat messages
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(self.config.prefix().to_string()),
                mention_as_prefix: false,

            //    // An edit tracker needs to be supplied here to make edit tracking in commands work
            //    edit_tracker: Some(poise::EditTracker::for_timespan(
            //        std::time::Duration::from_secs(3600 * 3),
            //    )),
                ..Default::default()
            },

            ..Default::default()
        };

        let cfg = Arc::clone(&self.config);
        poise::Framework::builder()
        .token(self.config.token())
        .intents(GatewayIntents::all())
        .options(options)
        .client_settings(move |c| {
            c.event_handler(Handler::new(cfg))
        })
        .user_data_setup(|_ctx, _data_about_bot, _framework| {
            Box::pin(async move {
                Ok(commands::Data {
                    command_counter: std::sync::Mutex::new(std::collections::HashMap::new()),
                })
            })
        })
        .run_autosharded()
        .await
        .expect("Client error");
    }
}
