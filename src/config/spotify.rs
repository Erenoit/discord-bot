use crate::{get_config, logger};
use std::{env, process};
use taplo::dom::Node;
use tokio::sync::RwLock;

pub(super) struct SpotifyConfig {
    client_id: String,
    client_secret: String,
    token: RwLock<Option<String>>,
}

impl SpotifyConfig {
    pub fn generate(config_file: &Node) -> Self {
        let client_id = get_value!(config_file, String, "BOT_SP_CLIENT_ID", "spotify"=>"client_id",
                                   "For Spotify support client ID is requared. Either set your client ID on the config file or disable Spotify support");
        let client_secret = get_value!(config_file, String, "BOT_SP_CLIENT_SECRET", "spotify"=>"client_secret",
                                   "For Spotify support client secret is requared. Either set your client secret on the config file or disable Spotify support");
        let token = RwLock::new(None);
        Self { client_id, client_secret, token }
    }

    #[inline(always)]
    pub const fn client(&self) -> (&String, &String) {
        (&self.client_id, &self.client_secret)
    }

    #[inline(always)]
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

