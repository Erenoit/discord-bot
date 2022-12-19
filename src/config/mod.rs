mod defaults;
#[macro_use]
mod macros;
mod spotify;

use crate::{config::{defaults::*, spotify::SpotifyConfig}, logger, server::Server};
use std::{collections::HashMap, env, fs, io::Write, process, sync::Arc};
use directories::ProjectDirs;
use dotenv;
use serenity::model::id::GuildId;
use songbird::Songbird;

use tokio::sync::RwLock;

pub struct Config {
    token: String,
    prefix: String,
    project_dirs: ProjectDirs,
    spotify: Option<SpotifyConfig>,
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

        logger::secondary_info("Token");
        let token = get_value!(config_file, String, "BOT_TOKEN", "general"=>"token", "Discord token couldn't found.");

        logger::secondary_info("Prefix");
        let prefix = get_value!(config_file, String, "BOT_PREFIX", "general"=>"prefix", PREFIX);

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

        logger::secondary_info("Servers HashMap");
        let servers = RwLock::new(HashMap::new());

        logger::secondary_info("Songbird");
        let songbird = Songbird::serenity();

        if spotify.is_none() {
            logger::warn("No Spotify config found");
        }

        Self { token, prefix, project_dirs, spotify, servers, songbird }
    }

    pub fn token(&self) -> &String {
        &self.token
    }

    pub fn prefix(&self) -> &String {
        &self.prefix
    }

    pub fn is_spotify_initialized(&self) -> bool {
        self.spotify.is_some()
    }

    pub fn spotify_client(&self) -> Option<(&String, &String)> {
        Some(self.spotify.as_ref()?.client())
    }

    pub async fn spotify_token(&self) -> Option<String> {
        Some(self.spotify.as_ref()?.token().await)
    }

    pub fn servers(&self) -> &RwLock<HashMap<GuildId, Server>> {
        &self.servers
    }

    pub fn songbird(&self) -> Arc<Songbird> {
        Arc::clone(&self.songbird)
    }
}

