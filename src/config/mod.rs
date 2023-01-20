mod cmd_arguments;
mod defaults;
#[macro_use]
mod macros;

mod database;
mod general;
mod spotify;
mod youtube;

use std::{collections::HashMap, env, fs, io::Write, process, sync::Arc};

use clap::Parser;
use directories::ProjectDirs;
use rocksdb::{DBWithThreadMode, MultiThreaded};
use serenity::model::id::GuildId;
use songbird::Songbird;
use tokio::sync::RwLock;

use crate::{
    config::{
        cmd_arguments::CMDArguments,
        database::DatabaseConfig,
        defaults::{ENABLE_DATABASE, ENABLE_SPOTIFY},
        general::GeneralConfig,
        spotify::SpotifyConfig,
        youtube::YouTubeConfig,
    },
    logger,
    server::Server,
};

#[non_exhaustive]
pub struct Config {
    general:  GeneralConfig,
    youtube:  YouTubeConfig,
    spotify:  Option<SpotifyConfig>,
    database: Option<DatabaseConfig>,
    servers:  RwLock<HashMap<GuildId, Server>>,
    songbird: Arc<Songbird>,
}

impl Config {
    pub fn generate() -> Self {
        let cmd_arguments = CMDArguments::parse();

        logger::info("Generating Project Directories");
        let project_dirs = ProjectDirs::from("com", "Erenoit", "The Bot").map_or_else(
            || {
                logger::error("Couldn't find config location");
                process::exit(1);
            },
            |p| p,
        );
        let config_file_path = cmd_arguments.cfg_file_path.unwrap_or(project_dirs.config_dir().join("config.toml"));
        if !config_file_path.exists() {
            fs::create_dir_all(config_file_path.parent().expect(
                "it is safe to assume that this will always have a parent because we used join",
            ))
            .expect("directory creation should not fail in normal circumstances");

            let mut config_file =
                fs::File::create(&config_file_path).expect("file creation should not fail");
            config_file
                .write_all(include_bytes!("../../examples/config.toml"))
                .expect("file is created just one line before this should not fail");
        }

        logger::info("Registering Configs");
        _ = dotenv::dotenv(); // It doesn't matter even if it fails
        let config_file = taplo::parser::parse(
            fs::read_to_string(config_file_path)
                .expect("config not found/cannot read")
                .as_str(),
        )
        .into_dom();

        logger::secondary_info("General");
        let general = GeneralConfig::generate(&config_file);

        logger::secondary_info("YouTube");
        let youtube = YouTubeConfig::generate(&config_file);

        logger::secondary_info("Spotify");
        let spotify = get_value!(config_file, bool, "BOT_ENABLE_SPOTIFY", "spotify"=>"enable", ENABLE_SPOTIFY).then(|| {
            SpotifyConfig::generate(&config_file)
        });

        logger::secondary_info("Database");
        let database = get_value!(config_file, bool, "BOT_ENABLE_DATABASE", "database"=>"enable", ENABLE_DATABASE).then(|| {
            DatabaseConfig::generate(&config_file, cmd_arguments.database_folder_path.unwrap_or(project_dirs.data_dir().join("database")))
        });

        logger::secondary_info("Servers HashMap");
        let servers = RwLock::new(HashMap::new());

        logger::secondary_info("Songbird");
        let songbird = Songbird::serenity();

        if spotify.is_none() {
            logger::warn("No Spotify config found");
        }

        if database.is_none() {
            logger::warn("Database is unavailable");
        }

        Self {
            general,
            youtube,
            spotify,
            database,
            servers,
            songbird,
        }
    }

    #[inline(always)]
    pub const fn token(&self) -> &String { self.general.token() }

    #[inline(always)]
    pub const fn prefix(&self) -> &String { self.general.prefix() }

    #[inline(always)]
    pub const fn auto_register_commands(&self) -> bool { self.general.auto_register_commands() }

    #[inline(always)]
    pub const fn vc_auto_change(&self) -> bool { self.general.vc_auto_change() }

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

    #[inline(always)]
    pub const fn database(&self) -> Option<&DBWithThreadMode<MultiThreaded>> {
        if let Some(db) = &self.database {
            Some(db.connection())
        } else {
            None
        }
    }

    #[inline(always)]
    pub const fn servers(&self) -> &RwLock<HashMap<GuildId, Server>> { &self.servers }

    #[inline(always)]
    pub fn songbird(&self) -> Arc<Songbird> { Arc::clone(&self.songbird) }
}
