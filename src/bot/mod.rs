pub mod commands;
mod event;

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
                prefix: Some(get_config().prefix().to_string()),
                mention_as_prefix: false,

                //// An edit tracker needs to be supplied here to make edit tracking in commands work
                //edit_tracker: Some(poise::EditTracker::for_timespan(
                //    std::time::Duration::from_secs(3600 * 3),
                //)),
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
        .user_data_setup(|_ctx, _data_about_bot, _framework| {
            Box::pin(async move {
                Ok(commands::Data)
            })
        })
        .run_autosharded()
        .await
        .expect("Client error");
    }
}
