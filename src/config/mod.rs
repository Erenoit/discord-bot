//! This module keeps state of the bot
//!
//! Even though it started as storing configuration about bot, now it is
//! responsible to store everything from configuration to music player state and
//! probably the most confusing file/module in this project yet it is
//! surprisingly straightforward.
//!
//! Main [`Config`] struct is divided into substructs/submodules. Each submodule
//! is responsible for storing configuration about a specific part of the bot.
//! Every struct is its own file to make it easier to find them.
//!
//! Also to not make every struct public their functions are re-exported in the
//! main [`Config`] struct.
//!
//! ## Why constructer is named `generate()`?
//! Every struct in config module and its submodules use `generate()` function
//! to generate their struct. This is because this structs only created once and
//! third party resources such as config files and environmental variables used
//! in their creation. Using different name for the constructor other than
//! `new()` is to make it clear that it is not a normal constructor.

/// Command line arguments for the bot
#[cfg(all(feature = "cmd", any(feature = "config_file", feature = "database")))]
mod cmd_arguments;
mod defaults;
#[macro_use]
mod macros;

#[cfg(feature = "database")]
mod database;
mod general;
mod message;
#[cfg(feature = "spotify")]
mod spotify;
#[cfg(feature = "music")]
mod youtube;

use std::{collections::HashMap, sync::Arc};
#[cfg(feature = "config_file")]
use std::{fs, io::Write};

#[cfg(any(feature = "config_file", feature = "database"))]
use anyhow::anyhow;
use anyhow::Result;
#[cfg(all(feature = "cmd", any(feature = "config_file", feature = "database")))]
use clap::Parser;
#[cfg(any(feature = "config_file", feature = "database"))]
use directories::ProjectDirs;
use serenity::model::id::GuildId;
#[cfg(feature = "music")]
use songbird::Songbird;
#[cfg(feature = "database")]
use sqlx::SqlitePool;
use tokio::sync::RwLock;
use tracing::trace;
#[cfg(any(feature = "spotify", feature = "database"))]
use tracing::warn;

#[cfg(all(feature = "cmd", any(feature = "config_file", feature = "database")))]
use crate::config::cmd_arguments::CMDArguments;
#[cfg(feature = "music")]
use crate::config::youtube::YouTubeConfig;
#[cfg(feature = "database")]
use crate::config::{database::DatabaseConfig, defaults::ENABLE_DATABASE};
#[cfg(feature = "spotify")]
use crate::config::{defaults::ENABLE_SPOTIFY, spotify::SpotifyConfig};
use crate::{
    config::{general::GeneralConfig, message::MessageConfig},
    server::Server,
};

/// Dummy type to use when `config_file` feature is not enabled. When
/// `config_file` feature is disabled, [`taplo`] crate is disabled. therefore,
/// there is no Node type, and this creates a compilation error.
#[cfg(not(feature = "config_file"))]
struct Node;

/// Main struct to store everything
#[non_exhaustive]
pub struct Config {
    /// General configuration
    general:  GeneralConfig,
    /// Message configuration
    message:  MessageConfig,
    /// `YouTube` configuration
    #[cfg(feature = "music")]
    youtube:  YouTubeConfig,
    /// `Spotify` configuration
    #[cfg(feature = "spotify")]
    spotify:  Option<SpotifyConfig>,
    /// Database configuration
    #[cfg(feature = "database")]
    database: Option<DatabaseConfig>,
    /// Connected servers
    servers:  RwLock<HashMap<GuildId, Arc<Server>>>,
    /// Songbird client
    #[cfg(feature = "music")]
    songbird: Arc<Songbird>,
}

impl Config {
    /// Generate [`Config`] from config sources
    #[expect(
        clippy::cognitive_complexity,
        reason = "False positive after migrating to tracing"
    )]
    pub fn generate() -> Result<Self> {
        #[cfg(all(feature = "cmd", any(feature = "config_file", feature = "database")))]
        let cmd_arguments = CMDArguments::parse();

        #[cfg(any(feature = "config_file", feature = "database"))]
        trace!("Creating Project Directories");
        #[cfg(any(feature = "config_file", feature = "database"))]
        let Some(project_dirs) = ProjectDirs::from("com", "Erenoit", "The Bot") else {
            return Err(anyhow!("Couldn't find config location"));
        };
        #[cfg(feature = "config_file")]
        let config_file_path = {
            #[cfg(feature = "cmd")]
            {
                cmd_arguments
                    .cfg_file_path
                    .unwrap_or_else(|| project_dirs.config_dir().join("config.toml"))
            }

            #[cfg(not(feature = "cmd"))]
            project_dirs.config_dir().join("config.toml")
        };
        #[cfg(feature = "config_file")]
        if !config_file_path.exists() {
            fs::create_dir_all(config_file_path.parent().ok_or_else(|| {
                anyhow!(
                    "it is safe to assume that this will always have a parent because we used join"
                )
            })?)?;

            fs::File::create(&config_file_path)?
                .write_all(include_bytes!("../../examples/config.toml"))?;
        }

        #[cfg(feature = "dotenv")]
        dotenvy::dotenv().ok();
        #[cfg(feature = "config_file")]
        let config_file =
            taplo::parser::parse(fs::read_to_string(config_file_path)?.as_str()).into_dom();
        #[cfg(not(feature = "config_file"))]
        let config_file = Node;

        trace!("Generating config: General");
        let general = GeneralConfig::generate(&config_file)?;

        trace!("Generating config: Message");
        let message = MessageConfig::generate(&config_file)?;

        #[cfg(feature = "music")]
        trace!("Generating config: YouTube");
        #[cfg(feature = "music")]
        let youtube = YouTubeConfig::generate(&config_file)?;

        #[cfg(feature = "spotify")]
        trace!("Generating config: Spotify");
        #[cfg(feature = "spotify")]
        let spotify = get_value!(config_file, bool, "BOT_ENABLE_SPOTIFY", "spotify"=>"enable", ENABLE_SPOTIFY)?.then_some(
            SpotifyConfig::generate(&config_file)?
        );

        #[cfg(feature = "database")]
        trace!("Generating config: Database");
        #[cfg(feature = "database")]
        let database = get_value!(config_file, bool, "BOT_ENABLE_DATABASE", "database"=>"enable", ENABLE_DATABASE)?.then_some({
            #[cfg(feature = "cmd")]
            let path = cmd_arguments.database_folder_path.unwrap_or_else(|| project_dirs.data_dir().to_path_buf());
            #[cfg(not(feature = "cmd"))]
            let path = project_dirs.data_dir().to_path_buf();

            DatabaseConfig::generate(&config_file, path)?
        });

        trace!("Creating Servers HashMap");
        let servers = RwLock::new(HashMap::new());

        #[cfg(feature = "music")]
        trace!("Creating Songbird");
        #[cfg(feature = "music")]
        let songbird = Songbird::serenity();

        #[cfg(feature = "spotify")]
        if spotify.is_none() {
            warn!("No Spotify config found");
        }

        #[cfg(feature = "database")]
        if database.is_none() {
            warn!("Database is unavailable");
        }

        Ok(Self {
            general,
            message,
            #[cfg(feature = "music")]
            youtube,
            #[cfg(feature = "spotify")]
            spotify,
            #[cfg(feature = "database")]
            database,
            servers,
            #[cfg(feature = "music")]
            songbird,
        })
    }

    /// Get `Discord` token for bot
    pub const fn token(&self) -> &String { self.general.token() }

    /// Get prefix for `prefix commands`
    pub const fn prefix(&self) -> &String { self.general.prefix() }

    /// Register `slash commands` to `Discord`
    pub const fn auto_register_commands(&self) -> bool { self.general.auto_register_commands() }

    /// Get `vc_auto_change` setting
    #[cfg(feature = "music")]
    pub const fn vc_auto_change(&self) -> bool { self.general.vc_auto_change() }

    /// Get `message_always_embed` setting
    ///
    /// WARNING: You do not need to use this setting. [`message!()`] macro
    /// already obeys this setting.
    ///
    /// [`message!()`]: crate::messager::message!()
    pub const fn message_always_embed(&self) -> bool { self.message.always_embed() }

    /// Get `message_randowm_colors` setting
    ///
    /// WARNING: You do not need to use this setting. [`message!()`] macro
    /// already obeys this setting.
    ///
    /// [`message!()`]: crate::messager::message!()
    pub const fn message_random_embed_colors(&self) -> bool { self.message.random_embed_colors() }

    /// Get `message_success_color` setting
    ///
    /// WARNING: You do not need to use this setting. [`message!()`] macro
    /// already obeys this setting.
    ///
    /// [`message!()`]: crate::messager::message!()
    pub const fn message_success_color(&self) -> u32 { self.message.success_color() }

    /// Get `message_normal_color` setting
    ///
    /// WARNING: You do not need to use this setting. [`message!()`] macro
    /// already obeys this setting.
    ///
    /// [`message!()`]: crate::messager::message!()
    pub const fn message_normal_color(&self) -> u32 { self.message.normal_color() }

    /// Get `message_error_color` setting
    ///
    /// WARNING: You do not need to use this setting. [`message!()`] macro
    /// already obeys this setting.
    ///
    /// [`message!()`]: crate::messager::message!()
    pub const fn message_error_color(&self) -> u32 { self.message.error_color() }

    /// Get `message_interaction_time_limit` setting
    pub const fn message_interaction_time_limit(&self) -> u64 {
        self.message.interaction_time_limit()
    }

    /// Get `youtube_search_count` setting
    #[cfg(feature = "music")]
    pub const fn youtube_search_count(&self) -> u8 { self.youtube.search_count() }

    /// Get `youtube_age_resricted` setting
    #[cfg(feature = "music")]
    pub const fn youtube_age_restricted(&self) -> bool { self.youtube.age_restricted() }

    /// Get `youtube_cookies` setting
    #[cfg(feature = "music")]
    pub fn youtube_cookies(&self) -> &str { self.youtube.cookies() }

    /// Chack if `Spotify` enabled
    #[cfg(feature = "spotify")]
    pub const fn is_spotify_initialized(&self) -> bool { self.spotify.is_some() }

    /// Get `Spotify` client credentials
    #[cfg(feature = "spotify")]
    pub fn spotify_client(&self) -> Option<(&String, &String)> {
        Some(self.spotify.as_ref()?.client())
    }

    /// Get `Spotify` token
    #[cfg(feature = "spotify")]
    pub async fn spotify_token(&self) -> Option<String> {
        Some(self.spotify.as_ref()?.token().await)
    }

    /// Get databse pool to interact with database
    ///
    /// WARNING: use [`bot::commands::macros::db_connection!()`] instead of this
    /// if you want database connection in [bot command].
    ///
    /// [`bot::commands::macros::db_connection!()`]: crate::bot::commands::macros::db_connection!()
    /// [bot_command]: crate::bot::commands
    #[cfg(feature = "database")]
    pub const fn database_pool(&self) -> Option<&SqlitePool> {
        if let Some(ref db) = self.database {
            Some(db.pool())
        } else {
            None
        }
    }

    /// Get database URL
    #[cfg(feature = "database")]
    pub const fn database_url(&self) -> Option<&String> {
        if let Some(ref db) = self.database {
            Some(db.url())
        } else {
            None
        }
    }

    /// Run database migrations to setup database
    #[cfg(feature = "database")]
    pub async fn run_database_migrations(&self) -> Result<()> {
        if let Some(ref db) = self.database {
            db.run_migrations().await?;
        }

        Ok(())
    }

    /// Get connected servers
    pub const fn servers(&self) -> &RwLock<HashMap<GuildId, Arc<Server>>> { &self.servers }

    /// Get main [`Songbird`] strcut
    #[cfg(feature = "music")]
    pub fn songbird(&self) -> Arc<Songbird> { Arc::clone(&self.songbird) }
}
