mod cmd_arguments;
mod defaults;
#[macro_use]
mod macros;

#[cfg(feature = "database")]
mod database;
mod general;
mod message;
mod spotify;
mod youtube;

use std::{collections::HashMap, env, fs, io::Write, sync::Arc};

use anyhow::{anyhow, Result};
use clap::Parser;
use directories::ProjectDirs;
use serenity::model::id::GuildId;
use songbird::Songbird;
#[cfg(feature = "database")]
use sqlx::SqlitePool;
use tokio::sync::RwLock;

#[cfg(feature = "database")]
use crate::config::{database::DatabaseConfig, defaults::ENABLE_DATABASE};
use crate::{
    config::{
        cmd_arguments::CMDArguments,
        defaults::ENABLE_SPOTIFY,
        general::GeneralConfig,
        message::MessageConfig,
        spotify::SpotifyConfig,
        youtube::YouTubeConfig,
    },
    server::Server,
};

#[non_exhaustive]
pub struct Config {
    general:  GeneralConfig,
    message:  MessageConfig,
    youtube:  YouTubeConfig,
    spotify:  Option<SpotifyConfig>,
    #[cfg(feature = "database")]
    database: Option<DatabaseConfig>,
    servers:  RwLock<HashMap<GuildId, Server>>,
    songbird: Arc<Songbird>,
}

impl Config {
    pub fn generate() -> Result<Self> {
        let cmd_arguments = CMDArguments::parse();

        log!(info, "Generating Project Directories");
        let Some(project_dirs) = ProjectDirs::from("com", "Erenoit", "The Bot") else {
            return Err(anyhow!("Couldn't find config location"));
        };
        let config_file_path = cmd_arguments
            .cfg_file_path
            .unwrap_or_else(|| project_dirs.config_dir().join("config.toml"));
        if !config_file_path.exists() {
            fs::create_dir_all(config_file_path.parent().ok_or_else(|| {
                anyhow!(
                    "it is safe to assume that this will always have a parent because we used join"
                )
            })?)?;

            fs::File::create(&config_file_path)?
                .write_all(include_bytes!("../../examples/config.toml"))?;
        }

        log!(info, "Registering Configs");
        #[cfg(feature = "dotenv")]
        drop(dotenv::dotenv()); // It doesn't matter even if it fails
        let config_file =
            taplo::parser::parse(fs::read_to_string(config_file_path)?.as_str()).into_dom();

        log!(info, ; "General");
        let general = GeneralConfig::generate(&config_file)?;

        log!(info, ; "Message");
        let message = MessageConfig::generate(&config_file)?;

        log!(info, ; "YouTube");
        let youtube = YouTubeConfig::generate(&config_file)?;

        log!(info, ; "Spotify");
        let spotify = get_value!(config_file, bool, "BOT_ENABLE_SPOTIFY", "spotify"=>"enable", ENABLE_SPOTIFY)?.then_some(
            SpotifyConfig::generate(&config_file)?
        );

        #[cfg(feature = "database")]
        log!(info, ; "Database");
        #[cfg(feature = "database")]
        let database = get_value!(config_file, bool, "BOT_ENABLE_DATABASE",
         "database"=>"enable", ENABLE_DATABASE)?
        .then_some(DatabaseConfig::generate(
            &config_file,
            cmd_arguments
                .database_folder_path
                .unwrap_or_else(|| project_dirs.data_dir().to_path_buf()),
        )?);

        log!(info, ; "Servers HashMap");
        let servers = RwLock::new(HashMap::new());

        log!(info, ; "Songbird");
        let songbird = Songbird::serenity();

        if spotify.is_none() {
            log!(warn, "No Spotify config found");
        }

        #[cfg(feature = "database")]
        if database.is_none() {
            log!(warn, "Database is unavailable");
        }

        Ok(Self {
            general,
            message,
            youtube,
            spotify,
            #[cfg(feature = "database")]
            database,
            servers,
            songbird,
        })
    }

    #[inline(always)]
    pub const fn token(&self) -> &String { self.general.token() }

    #[inline(always)]
    pub const fn prefix(&self) -> &String { self.general.prefix() }

    #[inline(always)]
    pub const fn auto_register_commands(&self) -> bool { self.general.auto_register_commands() }

    #[inline(always)]
    pub const fn vc_auto_change(&self) -> bool { self.general.vc_auto_change() }

    pub const fn message_always_embed(&self) -> bool { self.message.always_embed() }

    pub const fn message_random_embed_colors(&self) -> bool { self.message.random_embed_colors() }

    pub const fn message_success_color(&self) -> u32 { self.message.success_color() }

    pub const fn message_normal_color(&self) -> u32 { self.message.normal_color() }

    pub const fn message_error_color(&self) -> u32 { self.message.error_color() }

    pub const fn message_interaction_time_limit(&self) -> u64 {
        self.message.interaction_time_limit()
    }

    #[inline(always)]
    pub const fn youtube_search_count(&self) -> u8 { self.youtube.search_count() }

    #[inline(always)]
    pub const fn youtube_age_restricted(&self) -> bool { self.youtube.age_restricted() }

    #[inline(always)]
    pub const fn is_spotify_initialized(&self) -> bool { self.spotify.is_some() }

    #[inline(always)]
    pub fn spotify_client(&self) -> Option<(&String, &String)> {
        Some(self.spotify.as_ref()?.client())
    }

    #[inline(always)]
    pub async fn spotify_token(&self) -> Option<String> {
        Some(self.spotify.as_ref()?.token().await)
    }

    #[cfg(feature = "database")]
    #[inline(always)]
    pub const fn database_pool(&self) -> Option<&SqlitePool> {
        if let Some(db) = &self.database {
            Some(db.pool())
        } else {
            None
        }
    }

    pub async fn run_database_migrations(&self) -> Result<()> {
        if let Some(db) = &self.database {
            db.run_migrations().await?;
        }

        Ok(())
    }

    #[inline(always)]
    pub const fn servers(&self) -> &RwLock<HashMap<GuildId, Server>> { &self.servers }

    #[inline(always)]
    pub fn songbird(&self) -> Arc<Songbird> { Arc::clone(&self.songbird) }
}
