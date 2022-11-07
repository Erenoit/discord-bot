use crate::{get_config, logger, server::Server};
use std::{collections::HashMap, env, sync::Arc};
use dotenv;
use serenity::model::id::GuildId;
use songbird::Songbird;

use tokio::sync::RwLock;

pub struct Config {
    token: String,
    prefix: String,
    spotify: Option<SpotifyConfig>,
    servers: RwLock<HashMap<GuildId, Server>>,
    songbird: Arc<Songbird>,
}

impl Config {
    pub fn generate() -> Self {
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

        Self { token, prefix, spotify, servers, songbird }
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
