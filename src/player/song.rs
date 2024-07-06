//! Song struct and its methods.

use std::{collections::VecDeque, fmt::Display, iter};

use anyhow::{anyhow, Result};
use poise::futures_util::future::join_all;
use tokio::process::Command;

use super::yt_structs_new::{YoutubeVideo, YoutubeVideoPlaylist};
#[cfg(feature = "spotify")]
use crate::player::sp_structs::{
    SpotifyAlbumResponse,
    SpotifyArtistTopTracksResponse,
    SpotifyError,
    SpotifyPlaylistResponse,
    SpotifyTrackResponse,
};
use crate::{
    bot::Context,
    player::yt_structs::{YoutubeLink, YoutubeSearch},
};

/// User agent to use in requests
const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:111.0) Gecko/20100101 Firefox/111.0";
/// Base URL for spotify API
#[cfg(feature = "spotify")]
const SP_BASE_URL: &str = "https://api.spotify.com/v1";
/// Spotify market to use in Spotify API
#[cfg(feature = "spotify")]
const SP_MARKET: &str = "US";

/// Internal macro for getting id of the `YouTube` video URL. The main purpose
/// is reducing amount of copy paste code.
macro_rules! get_id {
    ($last_part:expr) => {
        $last_part
            .split('?')
            .next()
            .expect("At least one must be present")
    };
}

/// A uniform interface for getting song data from surce(s).
///
/// It also hadles when user input -such as choosing a song from search results-
/// needed.
///
/// Only supported sources are `YouTube` and `Spotify` at the moment.
#[derive(Clone)]
#[non_exhaustive]
pub struct Song {
    /// Title of the song.
    title:     String,
    /// YouTube video ID of the song.
    id:        String,
    /// Duration of the song.
    duration:  String,
    /// Username of the user who requested the song.
    user_name: String,
}

impl Song {
    /// Creates Song struct from given URL.
    ///
    /// Only `YouTube` and `Spotify` URLs are supported.
    ///
    /// If you want to search in in `YouTube` use [`Song::yt_search()`] instead.
    pub async fn new(ctx: &Context<'_>, song: String) -> Result<VecDeque<Self>> {
        let song = song.trim().to_owned();
        let user_name = ctx.author().name.clone();

        if song.starts_with("https://") || song.starts_with("http://") {
            if song.contains("youtube") {
                #[cfg(feature = "yt-dlp-fallback")]
                {
                    Self::youtube(song, user_name).await
                }

                #[cfg(not(feature = "yt-dlp-fallback"))]
                {
                    Self::youtube_new(song.as_str(), user_name.as_str()).await
                }
            } else if cfg!(feature = "spotify") && song.contains("spotify") {
                if get_config!().is_spotify_initialized() {
                    Self::spotify(song, user_name).await
                } else {
                    message!(error, ctx, ("Spotify is not initialized"); true);
                    Err(anyhow!("Spotify is not initialized"))
                }
            } else {
                message!(error, ctx, ("Unsupported music source"); true);
                Err(anyhow!("Unsupported music source"))
            }
        } else {
            #[cfg(feature = "yt-dlp-fallback")]
            {
                Self::search(ctx, song, user_name).await
            }

            #[cfg(not(feature = "yt-dlp-fallback"))]
            {
                Self::search_new(ctx, song.as_str(), user_name.as_str()).await
            }
        }
    }

    /// Takes search resoults for given string from `YouTube` and sends user to
    /// select one/all/none of them. Then returns the selected song(s).
    ///
    /// Uses new `YouTube` scrapper, but falls back to `yt-dlp` if new one
    /// fails.
    async fn search(ctx: &Context<'_>, song: String, user_name: String) -> Result<VecDeque<Self>> {
        if let Ok(res) = Self::search_new(ctx, &song, &user_name).await {
            res.map_or_else(|| Err(anyhow!("Selection failed/canceled")), Ok)
        } else if let Ok(res) = Self::search_old(ctx, &song, &user_name).await {
            log!(
                warn,
                "new scrapper failed, falling back to yt-dlp"
            );
            res.map_or_else(|| Err(anyhow!("Selection failed/canceled")), Ok)
        } else {
            Err(anyhow!("An error happened in search"))
        }
    }

    /// Sends GET request to `YouTube` as if it was searched in browser and
    /// scrapes the results.
    async fn search_new(
        ctx: &Context<'_>,
        song: &str,
        user_name: &str,
    ) -> Result<Option<VecDeque<Self>>> {
        let res = reqwest::Client::builder()
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:111.0) Gecko/20100101 Firefox/111.0",
            )
            .build()?
            .get(format!(
                "https://www.youtube.com/results?search_query={song}&sp=EgIQAQ%253D%253D"
            ))
            .send()
            .await?
            .text()
            .await?;

        let mut search_res = &res[res.find("ytInitialData").unwrap() + "ytInitialData = ".len() ..];
        search_res = &search_res[.. search_res.find("</script>").unwrap() - ";".len()];

        let list = serde_json::from_str::<YoutubeSearch>(search_res)?
            .contents
            .two_column_search_results_renderer
            .primary_contents
            .section_list_renderer
            .contents
            .into_iter()
            .filter_map(|contents| {
                contents.item_section_renderer.map(|item_renderer| {
                    item_renderer
                        .contents
                        .into_iter()
                        .filter_map(|item| {
                            item.video_renderer.map(|mut video| {
                                (
                                    video
                                        .title
                                        .runs
                                        .pop_front()
                                        .expect("At least one title should exist")
                                        .text,
                                    video.video_id,
                                    video.length_text.simple_text,
                                )
                            })
                        })
                        .collect::<Vec<_>>()
                })
            })
            .flatten()
            .take(get_config!().youtube_search_count().into())
            .collect::<Vec<_>>();

        let answer = selection!(list, *ctx, "Search", list, true);
        if answer == "success" {
            Ok(Some(
                list.into_iter()
                    .map(|(title, id, duration)| {
                        Self {
                            title,
                            id,
                            duration,
                            user_name: user_name.to_owned(),
                        }
                    })
                    .collect(),
            ))
        } else if answer != "danger" {
            Ok(Some(
                list.into_iter()
                    .filter(|(_, id, _)| id == &answer)
                    .take(1)
                    .map(|(title, id, duration)| {
                        Self {
                            title,
                            id,
                            duration,
                            user_name: user_name.to_owned(),
                        }
                    })
                    .collect(),
            ))
        } else {
            Ok(None)
        }
    }

    /// Uses old `yt-dlp` to search for given string in `YouTube`.
    #[cfg(feature = "yt-dlp-fallback")]
    async fn search_old(
        ctx: &Context<'_>,
        song: &str,
        user_name: &str,
    ) -> Result<Option<VecDeque<Self>>> {
        let Ok(res) = Command::new("yt-dlp")
            .args([
                "--flat-playlist",
                "--get-title",
                "--get-id",
                "--get-duration",
                song,
            ])
            .output()
            .await
        else {
            log!(error, "Command creation for yt-dlp failed");
            return Err(anyhow!("yt-dlp failed"));
        };

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
            Ok(Some(
                list.into_iter()
                    .map(|e| {
                        Self {
                            title:     e.0,
                            id:        e.1,
                            duration:  e.2,
                            user_name: user_name.to_owned(),
                        }
                    })
                    .collect(),
            ))
        } else if answer != "danger" {
            Ok(Some(
                list.into_iter()
                    .filter(|e| e.1 == answer)
                    .map(|e| {
                        Self {
                            title:     e.0,
                            id:        e.1,
                            duration:  e.2,
                            user_name: user_name.to_owned(),
                        }
                    })
                    .collect(),
            ))
        } else {
            Ok(None)
        }
    }

    // TODO: cannot open age restricted videos
    /// Takes `YouTube` URL and gets the song(s)
    async fn youtube(song: String, user_name: String) -> Result<VecDeque<Self>> {
        let res_new = Self::youtube_new(&song, &user_name).await;

        if res_new.is_err()
            && let Ok(res_old) = Self::youtube_old(&song, &user_name).await
        {
            log!(
                warn,
                "new scrapper failed, falling back to yt-dlp";
                "{}", (res_new.err().unwrap())
            );
            return Ok(res_old);
        }

        res_new
    }

    // TODO: Fix non-Playlist links
    // TODO: Playlist only links
    /// Sends GET request to `YouTube` as if it was requested from a browser and
    /// scrapes the result.
    async fn youtube_new(song: &str, user_name: &str) -> Result<VecDeque<Self>> {
        let res = reqwest::Client::builder()
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:111.0) Gecko/20100101 Firefox/111.0",
            )
            .build()?
            .get(song)
            .send()
            .await?
            .text()
            .await?;

        let mut song_list = VecDeque::new();

        let mut link_res = &res[res.find("ytInitialData").unwrap() + "ytInitialData = ".len() ..];
        link_res = &link_res[.. link_res.find("</script>").unwrap() - ";".len()];

        if song.contains("/watch?") {
            if song.contains("&list=") {
                let yt_initial_data =
                    &res[res.find("ytInitialData").unwrap() + "ytInitialData = ".len() ..];
                let yt_initial_data =
                    &yt_initial_data[.. yt_initial_data.find("</script>").unwrap() - ";".len()];

                let playlist_content =
                    serde_json::from_str::<YoutubeVideoPlaylist>(yt_initial_data)?
                        .contents
                        .two_column_watch_next_results
                        .playlist
                        .playlist
                        .contents;

                song_list.reserve(playlist_content.len());
                playlist_content.into_iter().for_each(|video| {
                    song_list.push_back(Song {
                        title:     video.playlist_panel_video_renderer.title.simple_text,
                        id:        video
                            .playlist_panel_video_renderer
                            .navigation_endpoint
                            .watch_endpoint
                            .video_id,
                        duration:  video.playlist_panel_video_renderer.length_text.simple_text,
                        user_name: user_name.to_owned(),
                    });
                });
            } else {
                let yt_initial_player_response =
                    &res[res.find("ytInitialPlayerResponse").unwrap()
                        + "ytInitialPlayerResponse = ".len() ..];
                let yt_initial_player_response = &yt_initial_player_response
                    [.. yt_initial_player_response.find(";var").unwrap()];

                let video_details =
                    serde_json::from_str::<YoutubeVideo>(yt_initial_player_response)?.video_details;

                song_list.push_back(Song {
                    title:     video_details.title,
                    id:        video_details.video_id,
                    duration:  video_details.length_seconds,
                    user_name: user_name.to_owned(),
                });
            }

            return Ok(song_list);
        } else if song.contains("/playlist?") {
            todo!("only playlist");
        } else {
            return Err(anyhow!("Unsupported YouTube link type"));
        }
        let video_two_col = serde_json::from_str::<YoutubeLink>(link_res)?
            .contents
            .two_column_watch_next_results;

        if let Some(playlist) = video_two_col.playlist {
            Ok(playlist
                .playlist
                .contents
                .into_iter()
                .map(|video| {
                    Self {
                        title:     video.title.simple_text,
                        id:        video.watch_endpoint.video_id,
                        duration:  video.length_text.simple_text,
                        user_name: user_name.to_owned(),
                    }
                })
                .collect())
        } else {
            todo!("Single video links are not supported yet")
        }
    }

    /// Uses old `yt-dlp` to get the song(s) from `YouTube` URL.
    #[cfg(feature = "yt-dlp-fallback")]
    async fn youtube_old(song: &str, user_name: &str) -> Result<VecDeque<Self>> {
        let Ok(res) = Command::new("yt-dlp")
            .args([
                "--flat-playlist",
                "--get-title",
                "--get-id",
                "--get-duration",
                song,
            ])
            .output()
            .await
        else {
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
                    user_name: user_name.to_owned(),
                }
            })
            .collect())
    }

    /// Takes `Spotify` URL and finds song(s) on `YouTube`.
    ///
    /// Adds ` lyrics` to the song name while searching on `YouTube` to avoid
    /// from the `official music video` version of the song. `official music
    /// video` versions generally has other parts in the video which
    /// is not relevant to the music.
    ///
    /// Artist, album and playlist, and track URLs are also supported.
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
            .await
        else {
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

    /// Get title of the song.
    pub fn title(&self) -> String { self.title.clone() }

    /// Get `YouTube` URL of the song.
    pub fn url(&self) -> String { self.id.clone() }

    /// Get duration of the song.
    pub fn duration(&self) -> String { self.duration.clone() }

    /// Get `Discord` user name of the person who requested the song.
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
