use crate::{get_config, logger, server::Server};
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
            config_file.write(include_str!("../examples/config.toml").as_bytes())
                .expect("file is created just one line before this should not fail");
        }

        logger::info("Registering Configs");
        _ = dotenv::dotenv();

        logger::secondary_info("Token");
        let token = env::var("BOT_TOKEN").expect("Couldn't find the token");

        logger::secondary_info("Prefix");
        let prefix = env::var("BOT_PREFIX").expect("Couldn't find the prefix");

        logger::secondary_info("Spotify");
        let spotify = SpotifyConfig::generate();

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
        if self.spotify.is_none() { return None; }
        Some(self.spotify.as_ref().unwrap().client())
    }

    pub async fn spotify_token(&self) -> Option<String> {
        if self.spotify.is_none() { return None; }
        Some(self.spotify.as_ref().unwrap().token().await)
    }

    pub fn servers(&self) -> &RwLock<HashMap<GuildId, Server>> {
        &self.servers
    }

    pub fn songbird(&self) -> Arc<Songbird> {
        Arc::clone(&self.songbird)
    }
}

struct SpotifyConfig {
    client_id: String,
    client_secret: String,
    token: RwLock<Option<String>>,
}

impl SpotifyConfig {
    fn generate() -> Option<Self> {
        let client_id = if let Ok(id) = env::var("BOT_SP_CLIENT_ID") { id } else { return None; };
        let client_secret = if let Ok(secret) = env::var("BOT_SP_CLIENT_SECRET") { secret } else { return None; };
        let token = RwLock::new(None);
        Some(Self { client_id, client_secret, token })
    }

    pub fn client(&self) -> (&String, &String) {
        (&self.client_id, &self.client_secret)
    }

    pub async fn token(&self) -> String {
        if self.token.read().await.is_none() {
            self.refresh_token().await
        }

        self.token.read().await.as_ref().unwrap().to_string()
    }

    async fn refresh_token(&self) {
        let mut write_lock = self.token.write().await;

        let (client_id, client_secret) = get_config().spotify_client().unwrap();
        let form = std::collections::HashMap::from([("grant_type", "client_credentials")]);

        let res = reqwest::Client::new()
            .post("https://accounts.spotify.com/api/token")
            .basic_auth(client_id, Some(client_secret))
            .form(&form)
            .send()
            .await;

        match res {
            Ok(r) => {
                if let Ok(j) = json::parse(&r.text().await.unwrap()) {
                    *write_lock = Some(j["access_token"].to_string());
                }
            }
            Err(why) => {
                logger::error("Couldn't get spotify token");
                logger::secondary_error(format!("{}", why));
            }
        }
    }
}
