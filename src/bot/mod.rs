mod commands;
mod event;

use event::Handler;
use serenity::model::{application::command::Command, gateway::GatewayIntents};
use songbird::SerenityInit;

pub use crate::bot::commands::Context;
use crate::{get_config, logger};

#[non_exhaustive]
pub struct Bot;

impl Bot {
    pub const fn new() -> Self { Self }

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
            .setup(|ctx, _data_about_bot, framework| {
                Box::pin(async move {
                    if !get_config().auto_register_commands() {
                        logger::warn("Slash Command Autogeneration Is Disabled");
                        return Ok(commands::Data);
                    }

                    logger::info("Registering Slash Commands:");
                    Command::set_global_application_commands(ctx, |b| {
                        let commands = &framework.options().commands;
                        *b = poise::builtins::create_application_commands(commands);
                        for command in commands {
                            logger::secondary_info(format!(
                                "{}: {}",
                                command.name,
                                command
                                    .description
                                    .as_ref()
                                    .expect("Every command should have description")
                            ));
                        }

                        b
                    })
                    .await?;
                    Ok(commands::Data)
                })
            })
            .run_autosharded()
            .await
            .expect("Client error");
    }
}

impl Default for Bot {
    fn default() -> Self { Self::new() }
}
