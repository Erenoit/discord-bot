//! Main module for the bot.
//!
//! This module contains the main struct for the bot, which is `Bot`. It also
//! handles everything related to discord except music. For music see
//! [`Player`].
//!
//! [`Player`]: crate::music::Player

mod commands;
mod event;

use event::Handler;
use serenity::model::{application::Command, gateway::GatewayIntents};
#[cfg(feature = "music")]
use songbird::serenity::SerenityInit;

#[cfg(feature = "music")]
pub use crate::bot::commands::Context;

/// The main struct for the bot.
///
/// Every interaction with `Discord` is done through this struct.
#[non_exhaustive]
pub struct Bot;

impl Bot {
    /// Creates a new instance of the bot.
    #[must_use]
    pub const fn new() -> Self { Self }

    /// Runs the bot.
    ///
    /// # Panics
    ///
    /// This method panics if it cannot run database mitigations or cannot
    /// connects to the `Discord`
    pub async fn run(&mut self) {
        #[cfg(feature = "database")]
        get_config!()
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
                prefix: Some(get_config!().prefix().to_string()),
                mention_as_prefix: false,
                execute_self_messages: false,
                ignore_bots: true,

                ..Default::default()
            },

            ..Default::default()
        };

        let framework = poise::Framework::builder()
            .options(options)
            .setup(|ctx, _data_about_bot, framework| {
                Box::pin(async move {
                    if !get_config!().auto_register_commands() {
                        log!(warn, "Slash Command Autogeneration Is Disabled");
                        return Ok(commands::Data);
                    }

                    log!(info, "Registering Slash Commands:");
                    Command::set_global_commands(ctx, {
                        let commands = &framework.options().commands;
                        let b = poise::builtins::create_application_commands(commands);
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
            .build();

        #[cfg(feature = "music")]
        {
            serenity::Client::builder(get_config!().token(), GatewayIntents::all())
                .framework(framework)
                .event_handler(Handler::new())
                .register_songbird_with(get_config!().songbird())
                .await
                .expect("Couldn't create a Client")
                .start()
                .await
                .expect("Couldn't start the Client");
        }
        #[cfg(not(feature = "music"))]
        {
            serenity::Client::builder(get_config!().token(), GatewayIntents::all())
                .framework(framework)
                .event_handler(Handler::new())
                .await
                .expect("Couldn't create a Client")
                .start_autosharded()
                .await
                .expect("Couldn't start the Client");
        }
    }
}

impl Default for Bot {
    fn default() -> Self { Self::new() }
}
