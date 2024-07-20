//! Main module for the bot.
//!
//! This module contains the main struct for the bot, which is `Bot`. It also
//! handles everything related to discord except music. For music see
//! [`Player`].
//!
//! [`Player`]: crate::music::Player

mod commands;
mod event;

use std::sync::Arc;

use event::Handler;
use reqwest::{Client, Url};
use serenity::model::{application::Command, gateway::GatewayIntents};
#[cfg(feature = "music")]
use songbird::serenity::SerenityInit;

#[cfg(feature = "music")]
pub use crate::bot::commands::Context;
use crate::cookie_jar::CookieJar;

/// User agent to use in requests
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:128.0) Gecko/20100101 Firefox/128.0";

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

        let reqwest_client = Self::create_reqwest_client();
        // Somehow it is moved inside the closure so, we need to clone it beforehand.
        let req_cli_clone = reqwest_client.clone();

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
                        return Ok(commands::Data { reqwest_client });
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
                    Ok(commands::Data { reqwest_client })
                })
            })
            .build();

        #[cfg(feature = "music")]
        {
            serenity::Client::builder(get_config!().token(), GatewayIntents::all())
                .framework(framework)
                .event_handler(Handler::new(req_cli_clone))
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
                .event_handler(Handler::new(req_cli_clone))
                .await
                .expect("Couldn't create a Client")
                .start_autosharded()
                .await
                .expect("Couldn't start the Client");
        }
    }

    /// Creates a new instance of [`reqwest::Client`] for global use.
    fn create_reqwest_client() -> Client {
        use reqwest::cookie::CookieStore;

        let reqwest_client_builder = Client::builder()
            .user_agent(USER_AGENT)
            .use_rustls_tls()
            .https_only(true);

        let cookie_jar = CookieJar::new();

        let url = "https://www.youtube.com"
            .parse::<Url>()
            .expect("Always works");

        let yt_cookies = get_config!().youtube_cookies();
        let saved_cookies = cookie_jar.cookies(&url);
        if !yt_cookies.is_empty() && saved_cookies.is_none() {
            let c = [reqwest::header::HeaderValue::from_str(yt_cookies).unwrap()];
            cookie_jar.set_cookies(&mut c.iter(), &url);
        }

        let reqwest_client_builder = reqwest_client_builder.cookie_provider(Arc::new(cookie_jar));

        reqwest_client_builder
            .build()
            .expect("TLS backend cannot be initialized")
    }
}

impl Default for Bot {
    fn default() -> Self { Self::new() }
}
