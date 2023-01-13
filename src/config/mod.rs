mod defaults;
mod general;
#[macro_use]
mod macros;
mod spotify;
mod youtube;

use crate::{config::{defaults::*, spotify::SpotifyConfig, youtube::YouTubeConfig, general::GeneralConfig}, logger, server::Server};
use std::{collections::HashMap, env, fs, io::Write, path::PathBuf, process, sync::Arc};
use directories::ProjectDirs;
use dotenv;
use rocksdb::{DBWithThreadMode, MultiThreaded, Options};
use serenity::model::id::GuildId;
use songbird::Songbird;
use tokio::sync::RwLock;

pub struct Config {
    general: GeneralConfig,
    youtube: YouTubeConfig,
    spotify: Option<SpotifyConfig>,
    database: Option<DBWithThreadMode<MultiThreaded>>,
    servers: RwLock<HashMap<GuildId, Server>>,
    songbird: Arc<Songbird>,
}

impl Config {
    pub fn generate() -> Self {
        logger::info("Generating Project Directories");
        let project_dirs = if let Some(p) = ProjectDirs::from("com", "Erenoit", "The Bot") { p }
            else {
                logger::error("Couldn't find config location");
                process::exit(1);
            };
        let config_file_path = project_dirs.config_dir().join("config.toml");
        if !config_file_path.exists() {
            fs::create_dir_all(config_file_path.parent()
                               .expect("it is safe to assume that this will always have a parent because we used join"))
                .expect("directory creation should not fail in normal circumstances");

            let mut config_file = fs::File::create(&config_file_path)
                .expect("file creation should not fail");
            config_file.write_all(include_str!("../../examples/config.toml").as_bytes())
                .expect("file is created just one line before this should not fail");
        }

        logger::info("Registering Configs");
        _ = dotenv::dotenv(); // It doesn't matter even if it fails
        let config_file = taplo::parser::parse(
            fs::read_to_string(config_file_path)
                .expect("config not found/cannot read")
                .as_str()).into_dom();

        logger::secondary_info("General");
        let general = {
            let token = get_value!(config_file, String, "BOT_TOKEN", "general"=>"token", "Discord token couldn't found.");
            let prefix = get_value!(config_file, String, "BOT_PREFIX", "general"=>"prefix", PREFIX);

            GeneralConfig::generate(token, prefix)
        };

        logger::secondary_info("YouTube");
        let youtube = {
            let search_count = get_value!(config_file, u8, "BOT_YT_SEARCH_COUNT", "youtube"=>"search_count", YT_SEARCH_COUNT);
            let age_restricted = get_value!(config_file, bool, "BOT_YT_AGE_RESTRICTED", "youtube"=>"age_restricted", YT_AGE_RESTRICTED);

            YouTubeConfig::generate(search_count, age_restricted)
        };

        logger::secondary_info("Spotify");
        let spotify = if get_value!(config_file, bool, "BOT_ENABLE_SPOTIFY", "spotify"=>"enable", ENABLE_SPOTIFY) {
            let client_id = get_value!(config_file, String, "BOT_SP_CLIENT_ID", "spotify"=>"client_id",
                                       "For Spotify support client ID is requared. Either set your client ID on the config file or disable Spotify support");
            let client_secret = get_value!(config_file, String, "BOT_SP_CLIENT_SECRET", "spotify"=>"client_secret",
                                       "For Spotify support client secret is requared. Either set your client secret on the config file or disable Spotify support");

            Some(SpotifyConfig::generate(client_id, client_secret))
        } else {
            None
        };

        logger:: secondary_info("Database");
        let database: Option<DBWithThreadMode<MultiThreaded>> =
            if get_value!(config_file, bool, "BOT_ENABLE_DATABASE", "database"=>"enable", ENABLE_DATABASE) {
                let default_path = project_dirs.data_dir().join("database");
                let path = get_value!(config_file, PathBuf, "BOT_DATABASE_LOCATION", "database"=>"location", default_path);

                if !path.exists() {
                    fs::create_dir_all(path.parent()
                                       .expect("it is safe to assume that this will always have a parent because we used join"))
                        .expect("directory creation should not fail in normal circumstances");
                }

                let mut options = Options::default();
                options.create_if_missing(true);
                options.create_missing_column_families(true);

                match DBWithThreadMode::open(&options, path) {
                    Ok(db) => Some(db),
                    Err(why) => {
                        logger::error("Couldn't open database.");
                        logger::secondary_error(why);
                        process::exit(1);
                    }
                }
            } else { None };

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

        Self { general, youtube, spotify, database, servers, songbird }
    }

    #[inline(always)]
    pub fn token(&self) -> &String {
        self.general.token()
    }

    #[inline(always)]
    pub fn prefix(&self) -> &String {
        self.general.prefix()
    }

    #[inline(always)]
    pub fn youtube_search_count(&self) -> u8 {
        self.youtube.search_count()
    }

    #[inline(always)]
    pub fn youtube_age_restricted(&self) -> bool {
        self.youtube.age_restricted()
    }

    #[inline(always)]
    pub fn is_spotify_initialized(&self) -> bool {
        self.spotify.is_some()
    }

    #[inline(always)]
    pub fn spotify_client(&self) -> Option<(&String, &String)> {
        Some(self.spotify.as_ref()?.client())
    }

    #[inline(always)]
    pub async fn spotify_token(&self) -> Option<String> {
        Some(self.spotify.as_ref()?.token().await)
    }

    #[inline(always)]
    pub fn database(&self) -> &Option<DBWithThreadMode<MultiThreaded>> {
        &self.database
    }

    #[inline(always)]
    pub fn servers(&self) -> &RwLock<HashMap<GuildId, Server>> {
        &self.servers
    }

    #[inline(always)]
    pub fn songbird(&self) -> Arc<Songbird> {
        Arc::clone(&self.songbird)
    }
}

