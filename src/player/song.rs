use std::{collections::VecDeque, fmt::Display, iter};

use anyhow::{anyhow, Result};
use poise::futures_util::future::join_all;
use tokio::process::Command;

use crate::bot::Context;
#[cfg(feature = "spotify")]
use crate::player::sp_structs::{
    SpotifyAlbumResponse,
    SpotifyArtistTopTracksResponse,
    SpotifyError,
    SpotifyPlaylistResponse,
    SpotifyTrackResponse,
};

const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:111.0) Gecko/20100101 Firefox/111.0";
#[cfg(feature = "spotify")]
const SP_BASE_URL: &str = "https://api.spotify.com/v1";
#[cfg(feature = "spotify")]
const SP_MARKET: &str = "US";

macro_rules! get_id {
    ($last_part:expr) => {
        $last_part
            .split('?')
            .next()
            .expect("At least one must be present")
    };
}

#[derive(Clone)]
#[non_exhaustive]
pub struct Song {
    title:     String,
    id:        String,
    duration:  String,
    user_name: String,
}

impl Song {
    pub async fn new(ctx: &Context<'_>, song: String) -> Result<VecDeque<Self>> {
        let song = song.trim().to_owned();
        let user_name = ctx.author().name.clone();

        if song.starts_with("https://") || song.starts_with("http://") {
            if song.contains("youtube") {
                Self::youtube(song, user_name).await
            } else if cfg!(feature = "spotify") && song.contains("spotify") {
                Self::spotify(song, user_name).await
            } else {
                message!(error, ctx, ("Unsupported music source"); true);
                Err(anyhow!("Unsupported music source"))
            }
        } else {
            Self::search(ctx, song, user_name).await
        }
    }

    async fn search(ctx: &Context<'_>, song: String, user_name: String) -> Result<VecDeque<Self>> {
        let res = Command::new("yt-dlp")
            .args([
                "--no-playlist",
                "--get-title",
                "--get-id",
                "--get-duration",
                &format!(
                    "ytsearch{}:{}",
                    get_config!().youtube_search_count(),
                    song
                ),
            ])
            .output()
            .await?;

        if !res.status.success() {
            log!(error, "YouTube data fetch with yt-dlp failed:"; "{}", (String::from_utf8(res.stderr).expect("Output must be valid UTF-8")));
            return Err(anyhow!("yt-dlp failed"));
        }

        let list = String::from_utf8_lossy(&res.stdout)
            .lines()
            .array_chunks::<3>()
            .map(|e| (e[0].to_owned(), e[1].to_owned(), e[2].to_owned()))
            .collect::<Vec<_>>();

        let answer = selection!(list, *ctx, "Search", list, true);
        if answer == "success" {
            Ok(list
                .into_iter()
                .map(|e| {
                    Self {
                        title:     e.0,
                        id:        e.1,
                        duration:  e.2,
                        user_name: user_name.clone(),
                    }
                })
                .collect::<VecDeque<_>>())
        } else if answer != "danger" {
            Ok(list
                .into_iter()
                .filter(|e| e.1 == answer)
                .map(|e| {
                    Self {
                        title:     e.0,
                        id:        e.1,
                        duration:  e.2,
                        user_name: user_name.clone(),
                    }
                })
                .collect::<VecDeque<_>>())
        } else {
            Err(anyhow!("Selection failed/canceled"))
        }
    }

    // TODO: yt-dlp is slow sometimes
    // TODO: cannot open age restricted videos
    async fn youtube(song: String, user_name: String) -> Result<VecDeque<Self>> {
        let Ok(res) = Command::new("yt-dlp")
            .args([
                "--flat-playlist",
                "--get-title",
                "--get-id",
                "--get-duration",
                &song,
            ])
            .output()
            .await else {
                log!(error, "Command creation for yt-dlp failed");
                return Err(anyhow!("yt-dlp failed"));
            };

        if !res.status.success() {
            log!(error, "YouTube data fetch with yt-dlp failed:"; "{}", (String::from_utf8(res.stderr).expect("Output must be valid UTF-8")));
            return Err(anyhow!("yt-dlp failed"));
        }

        Ok(String::from_utf8_lossy(&res.stdout)
            .split('\n')
            .array_chunks::<3>()
            .map(|e| {
                Self {
                    title:     e[0].to_owned(),
                    id:        e[1].to_owned(),
                    duration:  e[2].to_owned(),
                    user_name: user_name.clone(),
                }
            })
            .collect())
    }

    #[cfg(feature = "spotify")]
    pub async fn spotify(song: String, user_name: String) -> Result<VecDeque<Self>> {
        let Some(token) = get_config!().spotify_token().await else {
            return Err(anyhow!("Spotify token is not initialized"));
        };

        let (url_type, id, extra) = match song.split('/').take(5).collect::<Vec<_>>().as_slice() {
            ["https:", "", "open.spotify.com", "track", last] => ("tracks", get_id!(last), ""),
            ["https:", "", "open.spotify.com", "playlist", last] =>
                ("playlists", get_id!(last), ""),
            ["https:", "", "open.spotify.com", "album", last] => ("albums", get_id!(last), ""),
            ["https:", "", "open.spotify.com", "artist", last] =>
                ("artists", get_id!(last), "/top-tracks"),
            _ => return Err(anyhow!("Unsupported Spotify URL type")),
        };

        let Ok(res) = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .build()?
            .get(format!("{SP_BASE_URL}/{url_type}/{id}{extra}"))
            .bearer_auth(token)
            .query(&[("market", SP_MARKET)])
            .send()
            .await else {
                return Err(anyhow!("Couldn't connect to Spotify"));
            };

        if !res.status().is_success() {
            log!(error, "Spotify data fetch failed:"; "{}", (res.json::<SpotifyError>().await?.message));
            return Err(anyhow!("Couldn't connect to Spotify"));
        }

        let list = match url_type {
            "tracks" =>
                iter::once(res.json::<SpotifyTrackResponse>().await?)
                    .map(|track| track.name)
                    .collect::<Vec<_>>(),
            "playlists" =>
                res.json::<SpotifyPlaylistResponse>()
                    .await?
                    .tracks
                    .items
                    .into_iter()
                    .map(|track| track.track.name)
                    .collect::<Vec<_>>(),
            "albums" =>
                res.json::<SpotifyAlbumResponse>()
                    .await?
                    .tracks
                    .items
                    .into_iter()
                    .map(|track| track.name)
                    .collect::<Vec<_>>(),
            "artists" =>
                res.json::<SpotifyArtistTopTracksResponse>()
                    .await?
                    .tracks
                    .into_iter()
                    .map(|track| track.name)
                    .collect::<Vec<_>>(),
            _ => unreachable!("url_type cannot be anything else"),
        };

        Ok(join_all(list.into_iter().map(|song| {
            Command::new("yt-dlp")
                .args([
                    "--no-playlist",
                    "--get-title",
                    "--get-id",
                    "--get-duration",
                    &format!("ytsearch1:{song} lyrics"),
                ])
                .output()
        }))
        .await
        .into_iter()
        .filter(Result::is_ok)
        .map(|song| String::from_utf8_lossy(&song.expect("all is Ok").stdout).into_owned())
        .map(|song| {
            let mut sliced = song.split('\n').take(3);

            Self {
                title:     sliced.next().expect("Has 3 elements").to_owned(),
                id:        sliced.next().expect("Has 3 elements").to_owned(),
                duration:  sliced.next().expect("Has 3 elements").to_owned(),
                user_name: user_name.clone(),
            }
        })
        .collect())
    }

    pub fn title(&self) -> String { self.title.clone() }

    pub fn url(&self) -> String { self.id.clone() }

    pub fn duration(&self) -> String { self.duration.clone() }

    pub fn user_name(&self) -> String { self.user_name.clone() }
}

impl Display for Song {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} [{}] `requested by {}`",
            self.title(),
            self.duration(),
            self.user_name()
        )
    }
}
