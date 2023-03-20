mod commands;
mod event;

use event::Handler;
use serenity::model::{application::command::Command, gateway::GatewayIntents};
#[cfg(feature = "music")]
use songbird::SerenityInit;

pub use crate::bot::commands::Context;
use crate::get_config;

#[non_exhaustive]
pub struct Bot;

impl Bot {
    pub const fn new() -> Self { Self }

    pub async fn run(&mut self) {
        #[cfg(feature = "database")]
        get_config()
            .run_database_migrations()
            .await
            .expect("Couldn't setup the database");

        let options = poise::FrameworkOptions {
            commands: vec![
                commands::others::help::help(),
                commands::others::register::register(),
                commands::others::ping::ping(),
                commands::entertainment::sus::sus(),
                commands::entertainment::meme::meme(),
                #[cfg(feature = "music")]
                commands::music::join::join(),
                #[cfg(feature = "music")]
                commands::music::leave::leave(),
                #[cfg(feature = "music")]
                commands::music::play::play(),
                #[cfg(feature = "music")]
                commands::music::stop::stop(),
                #[cfg(feature = "music")]
                commands::music::skip::skip(),
                #[cfg(feature = "music")]
                commands::music::repeat::repeat(),
                #[cfg(feature = "database")]
                commands::music::music::music(),
                #[cfg(feature = "music")]
                commands::music::queue::queue(),
                #[cfg(feature = "music")]
                commands::music::clear::clear(),
                #[cfg(feature = "music")]
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
                #[cfg(feature = "music")]
                {
                    c.event_handler(Handler::new())
                        .register_songbird_with(get_config().songbird())
                }

                #[cfg(not(feature = "music"))]
                c.event_handler(Handler::new())
            })
            .setup(|ctx, _data_about_bot, framework| {
                Box::pin(async move {
                    if !get_config().auto_register_commands() {
                        log!(warn, "Slash Command Autogeneration Is Disabled");
                        return Ok(commands::Data);
                    }

                    log!(info, "Registering Slash Commands:");
                    Command::set_global_application_commands(ctx, |b| {
                        let commands = &framework.options().commands;
                        *b = poise::builtins::create_application_commands(commands);
                        for command in commands {
                            log!(info, ; "{}: {}", (command.name),
                                (command
                                    .description
                                    .as_ref()
                                    .expect("Every command should have description"))
                            );
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
