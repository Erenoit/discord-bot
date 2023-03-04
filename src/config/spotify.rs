use std::time::Instant;

use anyhow::Result;
#[cfg(feature = "config_file")]
use taplo::dom::Node;
use tokio::sync::RwLock;

#[cfg(not(feature = "config_file"))]
use crate::config::Node;

#[non_exhaustive]
pub(super) struct SpotifyConfig {
    client_id:     String,
    client_secret: String,
    token:         RwLock<Option<String>>,
    last_refresh:  RwLock<Option<Instant>>,
}

impl SpotifyConfig {
    const REFRESH_TIME: u64 = 3500;

    pub fn generate(config_file: &Node) -> Result<Self> {
        let client_id = get_value!(config_file, String, "BOT_SP_CLIENT_ID", "spotify"=>"client_id",
                                   "For Spotify support client ID is requared. Either set your client ID on the config file or disable Spotify support")?;
        let client_secret = get_value!(config_file, String, "BOT_SP_CLIENT_SECRET", "spotify"=>"client_secret",
                                   "For Spotify support client secret is requared. Either set your client secret on the config file or disable Spotify support")?;
        let token = RwLock::new(None);
        let last_refresh = RwLock::new(None);

        Ok(Self {
            client_id,
            client_secret,
            token,
            last_refresh,
        })
    }

    #[inline(always)]
    pub const fn client(&self) -> (&String, &String) { (&self.client_id, &self.client_secret) }

    #[inline(always)]
    pub async fn token(&self) -> String {
        if self.token.read().await.is_none()
            || self
                .last_refresh
                .read()
                .await
                .expect("Should be Some")
                .elapsed()
                .as_secs()
                >= Self::REFRESH_TIME
        {
            self.refresh_token().await;
        }

        // TODO: remove this copy
        self.token
            .read()
            .await
            .as_ref()
            .expect("This can't be None at this point")
            .to_string()
    }

    async fn refresh_token(&self) {
        let mut write_lock_token = self.token.write().await;
        let mut write_lock_last_refresh = self.last_refresh.write().await;

        let form = std::collections::HashMap::from([("grant_type", "client_credentials")]);

        let res = reqwest::Client::new()
            .post("https://accounts.spotify.com/api/token")
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .form(&form)
            .send()
            .await;

        match res {
            Ok(r) =>
                if let Ok(j) = json::parse(&r.text().await.unwrap()) {
                    *write_lock_last_refresh = Some(Instant::now());
                    *write_lock_token = Some(j["access_token"].to_string());
                },
            Err(why) => {
                log!(error, "Couldn't get spotify token"; "{why}");
            },
        }
    }
}
