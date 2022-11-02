use crate::{logger, messager};
use std::fmt::Display;
use anyhow::{anyhow, Result};
use tokio::process::Command;

#[derive(Clone)]
pub(super) struct Song {
    title: String,
    url: String,
    length: String,
    user_name: String,
}

impl Song {
    pub async fn new<A, B>(url: A, user_name: B) -> Result<Self>
    where
        A: Into<String>,
        B: Into<String>
    {
        let s = {
            let u = url.into();

            if u.contains("youtube.com") {
                Self::yt_url(u).await?
            } else if u.contains("spotify.com") {
                Self::sp_url(u).await?
            } else {
                return Err(anyhow!("Unsupported source"));
            }
        };

        Ok(Self {
            title: s.0,
            url: s.1,
            length: s.2,
            user_name: user_name.into(),
        })
    }

        pub async fn from_playlist<A, B>(url: A, user_name: B) -> Result<Vec<Self>>
    where
        A: Into<String>,
        B: Into<String>
    {
        let u = url.into();
        let usr = user_name.into();

        Ok(if u.contains("youtube.com") {
            Self::yt_playlist(u).await?
        } else if u.contains("spotify.com") {
            Self::sp_playlist(u).await?
        } else {
            return Err(anyhow!("Unsupported source"));
        }.into_iter()
        .map(|e| {
            return Self {
                title: e.0,
                url: e.1,
                length: e.2,
                user_name: usr.clone(),
            }
        })
        .collect())
    }

    // TODO: youtube-dl is slow sometimes
    // TODO: cannot open age restricted videos
    #[inline(always)]
    async fn yt_url(url: String) -> Result<(String, String, String)> {
        if let Ok(res) = Command::new("youtube-dl")
            .args(["--get-title", "--get-id", "--get-duration", &url])
            .output().await
            {
                if !res.status.success() {
                    logger::error("YouTube data fetch with youtube-dl failed:");
                    logger::secondary_error(String::from_utf8(res.stderr).expect("Output must be valid UTF-8"));
                    return Err(anyhow!("youtube-dl failed"));
                }

                let splited_res: Vec<String> = String::from_utf8(res.stdout)
                    .expect("Output must be valid UTF-8")
                    .split("\n")
                    .map(|e| e.to_string())
                    .collect();

                let title    = splited_res.get(0);
                let id       = splited_res.get(1);
                let duration = splited_res.get(2);

                if title.is_none() || id.is_none() || duration.is_none() {
                    logger::error("Somehow youtube-dl returned less data");
                    return Err(anyhow!("youtube-dl failed"));
                }

                Ok((title.unwrap().to_string(),
                    format!("https://youtube.com/watch?v={}", id.unwrap()),
                    duration.unwrap().to_string()))
            } else {
                logger::error("Command creation for youtube-dl failed");
                Err(anyhow!("youtube-dl failed"))
            }
    }

    #[inline(always)]
    async fn yt_playlist(url: String) -> Result<Vec<(String, String, String)>> {
        if let Ok(res) = Command::new("youtube-dl")
            .args(["--flat-playlist","--get-title", "--get-id", "--get-duration", &url])
            .output().await
            {
                if !res.status.success() {
                    logger::error("YouTube data fetch with youtube-dl failed:");
                    logger::secondary_error(String::from_utf8(res.stderr).expect("Output must be valid UTF-8"));
                    return Err(anyhow!("youtube-dl failed"));
                }

                let splited_res: Vec<String> = String::from_utf8(res.stdout)
                    .expect("Output must be valid UTF-8")
                    .split("\n")
                    .filter(|e| !e.is_empty())
                    .map(|e| e.to_string())
                    .collect();

                if splited_res.len() % 3 != 0 {
                    logger::error("youtube-dl returned wrong number of arguments");
                    return Err(anyhow!("Output must be dividable by 3"))
                }

                let mut list: Vec<(String, String, String)> = Vec::with_capacity(splited_res.len() / 3);

                for i in 0 .. splited_res.len() / 3 {
                    list.push((splited_res.get(i * 3 + 0).unwrap().to_string(),
                               splited_res.get(i * 3 + 1).unwrap().to_string(),
                               splited_res.get(i * 3 + 2).unwrap().to_string()))
                }

                Ok(list)
            } else {
                logger::error("Command creation for youtube-dl failed");
                Err(anyhow!("youtube-dl failed"))
            }
    }

    #[inline(always)]
    async fn sp_url(url: String) -> Result<(String, String, String)> {
        todo!("Handle Spotify URL")
    }

    #[inline(always)]
    async fn sp_playlist(url: String) -> Result<Vec<(String, String, String)>> {
        todo!("Handle Spotify playlist")
    }

    #[inline(always)]
    pub fn title(&self) -> String {
        self.title.clone()
    }

    #[inline(always)]
    pub fn url(&self) -> String {
        self.url.clone()
    }

    #[inline(always)]
    pub fn length(&self) -> String {
        self.length.clone()
    }

    #[inline(always)]
    pub fn user_name(&self) -> String {
        self.user_name.clone()
    }
}

impl Display for Song {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.title, self.length)
    }
}

