use std::{collections::VecDeque, fmt::Display, iter};

use anyhow::{anyhow, Result};
use poise::futures_util::future::join_all;
use tokio::process::Command;

#[cfg(feature = "spotify")]
use crate::player::sp_structs::{
    SpotifyAlbumResponse,
    SpotifyArtistTopTracksResponse,
    SpotifyError,
    SpotifyPlaylistResponse,
    SpotifyTrackResponse,
};
use crate::{bot::Context, get_config};

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
    pub async fn new<S: Display + Send>(ctx: &Context<'_>, song: S) -> Result<VecDeque<Self>> {
        let song = song.to_string();
        let user_name = ctx.author().name.clone();

        if song.starts_with("http://") || song.starts_with("https://") {
            if song.contains("youtube") {
                if song.contains("list") {
                    Self::yt_url(song, user_name).await
                } else {
                    Self::yt_playlist(song, user_name).await
                }
            } else if cfg!(feature = "spotify") && song.contains("spotify") {
                Self::spotify(song, user_name).await
            } else {
                message!(error, ctx, ("Unsupported music source"); true);
                Err(anyhow!("Unsupported music source"))
            }
        } else {
            Self::yt_search(ctx, song, user_name).await
        }
    }

    pub async fn yt_search(
        ctx: &Context<'_>,
        song: String,
        user_name: String,
    ) -> Result<VecDeque<Self>> {
        // TODO: change something faster than yt-dlp
        // TODO: clean this code
        let search_count = 5;
        let out = Command::new("yt-dlp")
            .args([
                "--no-playlist",
                "--get-title",
                "--get-id",
                "--get-duration",
                &format!("ytsearch{search_count}:{song}"),
            ])
            .output()
            .await?;

        let list = String::from_utf8(out.stdout).unwrap();
        let list_seperated = list.split('\n').collect::<Vec<_>>();

        let mut l: Vec<(String, String)> = Vec::with_capacity(search_count);
        for i in 0 .. search_count {
            l.push((
                list_seperated[i * 3].to_owned(),
                list_seperated[i * 3 + 1].to_owned(),
            ));
        }

        let answer = selection!(list, *ctx, "Search", l, true);
        if answer == "success" {
            let mut return_vec = VecDeque::with_capacity(search_count);
            for i in 0 .. search_count {
                return_vec.push_back(Self {
                    title:     list_seperated[i * 3].to_owned(),
                    id:        list_seperated[i * 3].to_owned(),
                    duration:  list_seperated[i * 3].to_owned(),
                    user_name: user_name.clone(),
                });
            }
            Ok(return_vec)
        } else if answer != "danger" {
            let mut return_vec = VecDeque::with_capacity(1);
            for i in 0 .. search_count {
                if *list_seperated[i * 3 + 1] == answer {
                    return_vec.push_back(Self {
                        title: list_seperated[i * 3].to_owned(),
                        id: list_seperated[i * 3 + 1].to_owned(),
                        duration: list_seperated[i * 3 + 2].to_owned(),
                        user_name,
                    });
                    break;
                }
            }
            Ok(return_vec)
        } else {
            Err(anyhow!("Selection failed/canceled"))
        }
    }

    // TODO: yt-dlp is slow sometimes
    // TODO: cannot open age restricted videos
    async fn yt_url(song: String, user_name: String) -> Result<VecDeque<Self>> {
        if let Ok(res) = Command::new("yt-dlp")
            .args(["--get-title", "--get-id", "--get-duration", &song])
            .output()
            .await
        {
            if !res.status.success() {
                log!(error, "YouTube data fetch with yt-dlp failed:"; "{}", (String::from_utf8(res.stderr).expect("Output must be valid UTF-8")));
                return Err(anyhow!("yt-dlp failed"));
            }

            let splited_res: Vec<String> = String::from_utf8(res.stdout)
                .expect("Output must be valid UTF-8")
                .split('\n')
                .map(std::string::ToString::to_string)
                .collect();

            let title = splited_res.get(0);
            let id = splited_res.get(1);
            let duration = splited_res.get(2);

            if title.is_none() || id.is_none() || duration.is_none() {
                log!(error, "Somehow yt-dlp returned less data");
                return Err(anyhow!("yt-dlp failed"));
            }

            let mut return_vec = VecDeque::with_capacity(1);
            return_vec.push_back(Self {
                title: title.unwrap().clone(),
                id: id.unwrap().clone(),
                duration: duration.unwrap().clone(),
                user_name,
            });
            Ok(return_vec)
        } else {
            log!(error, "Command creation for yt-dlp failed");
            Err(anyhow!("yt-dlp failed"))
        }
    }

    async fn yt_playlist(song: String, user_name: String) -> Result<VecDeque<Self>> {
        if let Ok(res) = Command::new("yt-dlp")
            .args([
                "--flat-playlist",
                "--get-title",
                "--get-id",
                "--get-duration",
                &song,
            ])
            .output()
            .await
        {
            if !res.status.success() {
                log!(error, "YouTube data fetch with yt-dlp failed:"; "{}", (String::from_utf8(res.stderr).expect("Output must be valid UTF-8")));
                return Err(anyhow!("yt-dlp failed"));
            }

            let splited_res: Vec<String> = String::from_utf8(res.stdout)
                .expect("Output must be valid UTF-8")
                .split('\n')
                .filter(|e| !e.is_empty())
                .map(std::string::ToString::to_string)
                .collect();

            if splited_res.len() % 3 != 0 {
                log!(error, "yt-dlp returned wrong number of arguments");
                return Err(anyhow!("Output must be dividable by 3"));
            }

            let mut return_vec: VecDeque<Self> = VecDeque::with_capacity(splited_res.len() / 3);

            for i in 0 .. splited_res.len() / 3 {
                return_vec.push_back(Self {
                    title:     splited_res.get(i * 3).unwrap().to_string(),
                    id:        splited_res.get(i * 3 + 1).unwrap().to_string(),
                    duration:  splited_res.get(i * 3 + 2).unwrap().to_string(),
                    user_name: user_name.clone(),
                });
            }

            Ok(return_vec)
        } else {
            log!(error, "Command creation for yt-dlp failed");
            Err(anyhow!("yt-dlp failed"))
        }
    }

    #[cfg(feature = "spotify")]
    pub async fn spotify(song: String, user_name: String) -> Result<VecDeque<Self>> {
        let Some(token) = get_config().spotify_token().await else {
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
