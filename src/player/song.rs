//! Song struct and its methods.

use std::{collections::VecDeque, fmt::Display};

use anyhow::{anyhow, Result};
#[cfg(feature = "spotify")]
use poise::futures_util::future::join_all;
use reqwest::{
    header::{HeaderMap, HeaderName},
    Client,
    Url,
};
#[cfg(feature = "yt-dlp-fallback")]
use songbird::input::YoutubeDl;
use songbird::input::{HttpRequest, Input};
#[cfg(feature = "yt-dlp-fallback")]
use tokio::process::Command;

#[cfg(feature = "spotify")]
use crate::request::sp_structs::{
    SpotifyAlbum,
    SpotifyArtistTopTracks,
    SpotifyError,
    SpotifyPlaylist,
    SpotifyTrack,
};
use crate::{
    bot::Context,
    request::{
        yt_structs::{
            Format,
            YoutubePlayer,
            YoutubePlaylist,
            YoutubeSearch,
            YoutubeVideo,
            YoutubeVideoPlaylist,
        },
        USER_AGENT,
    },
};

/// Base URL for spotify API
#[cfg(feature = "spotify")]
const SP_BASE_URL: &str = "https://api.spotify.com/v1";
/// Spotify market to use in Spotify API
#[cfg(feature = "spotify")]
const SP_MARKET: &str = "US";

/// Internal macro for getting id of the `YouTube` video URL. The main purpose
/// is reducing amount of copy paste code.
#[cfg(feature = "spotify")]
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
    /// `YouTube` video ID of the song.
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
    pub async fn new(
        ctx: &Context<'_>,
        reqwest_client: &Client,
        song: String,
    ) -> Result<VecDeque<Self>> {
        let song = song.trim().to_owned();
        let user_name = &ctx.author().name;

        if song.starts_with("https://") || song.starts_with("http://") {
            // TODO: short youtube links
            if song.contains("youtube") {
                Self::youtube(reqwest_client, song, user_name).await
            } else if song.contains("spotify") {
                #[cfg(feature = "spotify")]
                if get_config!().is_spotify_initialized() {
                    Self::spotify(ctx, reqwest_client, song, user_name).await
                } else {
                    message!(error, ctx, ("Spotify is not initialized"); true);
                    Err(anyhow!("Spotify is not initialized"))
                }
                #[cfg(not(feature = "spotify"))]
                {
                    message!(error, ctx, ("Spotify support is not enabled"); true);
                    Err(anyhow!("Spotify support is not enabled"))
                }
            } else {
                message!(error, ctx, ("Unsupported music source"); true);
                Err(anyhow!("Unsupported music source"))
            }
        } else {
            let search_count = get_config!().youtube_search_count();
            Self::search(ctx, reqwest_client, song, user_name, search_count).await
        }
    }

    /// Takes search resoults for given string from `YouTube` and sends user to
    /// select one/all/none of them. Then returns the selected song(s).
    ///
    /// If `search_count` is 1, returns the first reault without asking to
    /// Discord user
    ///
    /// Uses new `YouTube` scrapper, but falls back to `yt-dlp` if new one
    /// fails.
    async fn search(
        ctx: &Context<'_>,
        reqwest_client: &Client,
        song: String,
        user_name: &str,
        search_count: u8,
    ) -> Result<VecDeque<Self>> {
        if search_count == 0 {
            return Ok(VecDeque::new());
        }

        let res_new = Self::search_new(
            ctx,
            reqwest_client,
            &song,
            user_name,
            search_count,
        )
        .await;

        if let Ok(songs) = res_new {
            return Ok(songs);
        }

        #[cfg(feature = "yt-dlp-fallback")]
        if let Ok(res) = Self::search_old(ctx, &song, user_name, search_count).await {
            log!(
                warn,
                "new scrapper failed, falling back to yt-dlp";
                "{}", (res_new.err().expect("Its already an error"))
            );
            return Ok(res);
        }

        Err(anyhow!("An error happened in search"))
    }

    /// Sends GET request to `YouTube` as if it was searched in browser and
    /// scrapes the results.
    ///
    /// If `search_count` is 1, returns the first reault without asking to
    /// Discord user
    async fn search_new(
        ctx: &Context<'_>,
        reqwest_client: &Client,
        song: &str,
        user_name: &str,
        search_count: u8,
    ) -> Result<VecDeque<Self>> {
        let url = Url::parse_with_params("https://www.youtube.com/results", &[(
            "search_query",
            song,
        )])?;

        let res = reqwest_client.get(url).send().await?.text().await?;

        let mut search_res = &res[res
            .find("ytInitialData")
            .ok_or_else(|| anyhow!("Parse error"))?
            + "ytInitialData = ".len() ..];
        search_res = &search_res[.. search_res
            .find("</script>")
            .ok_or_else(|| anyhow!("Parse error"))?
            - ";".len()];

        let list = sonic_rs::from_str::<YoutubeSearch>(search_res)?
            .contents
            .two_column_search_results_renderer
            .primary_contents
            .section_list_renderer
            .contents
            .into_iter()
            .filter_map(|contents| contents.item_section_renderer)
            .flat_map(|item_renderer| item_renderer.contents)
            .filter_map(|item| item.video_renderer)
            .take(search_count as usize)
            .map(|mut video| {
                (
                    std::mem::take(&mut video.title.runs[0].text),
                    video.video_id,
                    video.length_text.simple_text,
                )
            })
            .collect::<Vec<_>>();

        if search_count == 1 {
            return Ok(VecDeque::from([list
                .into_iter()
                .map(|(title, id, duration)| {
                    Self {
                        title,
                        id,
                        duration,
                        user_name: user_name.to_owned(),
                    }
                })
                .next()
                .ok_or_else(|| {
                    anyhow!("No result found on the YouTube")
                })?]));
        }

        let answer = selection!(list, *ctx, "Search", list, true);
        if answer == "success" {
            Ok(list
                .into_iter()
                .map(|(title, id, duration)| {
                    Self {
                        title,
                        id,
                        duration,
                        user_name: user_name.to_owned(),
                    }
                })
                .collect())
        } else if answer != "danger" {
            #[expect(clippy::pattern_type_mismatch, reason = "Couldn't solve")]
            Ok(VecDeque::from([list
                .into_iter()
                .find(|(_, id, _)| id == &answer)
                .map(|(title, id, duration)| {
                    Self {
                        title,
                        id,
                        duration,
                        user_name: user_name.to_owned(),
                    }
                })
                .expect("Cannot fail")]))
        } else {
            Ok(VecDeque::new())
        }
    }

    /// Uses old `yt-dlp` to search for given string in `YouTube`.
    ///
    /// If `search_count` is 1, returns the first reault without asking to
    /// Discord user
    #[cfg(feature = "yt-dlp-fallback")]
    async fn search_old(
        ctx: &Context<'_>,
        song: &str,
        user_name: &str,
        search_count: u8,
    ) -> Result<VecDeque<Self>> {
        let Ok(res) = Command::new("yt-dlp")
            .args([
                "--flat-playlist",
                "--get-title",
                "--get-id",
                "--get-duration",
                &format!("ytsearch{}:{}", search_count, song,),
            ])
            .output()
            .await
        else {
            log!(error, "Command creation for yt-dlp failed");
            return Err(anyhow!("yt-dlp failed"));
        };

        if !res.status.success() {
            log!(error, "YouTube data fetch with yt-dlp failed:"; "{}", (String::from_utf8_lossy(&res.stderr)));
            return Err(anyhow!("yt-dlp failed"));
        }

        let list = String::from_utf8_lossy(&res.stdout)
            .lines()
            .array_chunks::<3>()
            .map(|e| (e[0].to_owned(), e[1].to_owned(), e[2].to_owned()))
            .collect::<Vec<_>>();

        if search_count == 1 {
            return Ok(VecDeque::from([list
                .into_iter()
                .map(|(title, id, duration)| {
                    Self {
                        title,
                        id,
                        duration,
                        user_name: user_name.to_owned(),
                    }
                })
                .next()
                .ok_or_else(|| {
                    anyhow!("No result found on the YouTube")
                })?]));
        }

        let answer = selection!(list, *ctx, "Search", list, true);
        if answer == "success" {
            Ok(list
                .into_iter()
                .map(|e| {
                    Self {
                        title:     e.0,
                        id:        e.1,
                        duration:  e.2,
                        user_name: user_name.to_owned(),
                    }
                })
                .collect())
        } else if answer != "danger" {
            Ok(list
                .into_iter()
                .filter(|e| e.1 == answer)
                .map(|e| {
                    Self {
                        title:     e.0,
                        id:        e.1,
                        duration:  e.2,
                        user_name: user_name.to_owned(),
                    }
                })
                .collect())
        } else {
            Ok(VecDeque::new())
        }
    }

    // TODO: cannot open age restricted videos
    /// Takes `YouTube` URL and gets the song(s)
    async fn youtube(
        reqwest_client: &Client,
        song: String,
        user_name: &str,
    ) -> Result<VecDeque<Self>> {
        let res_new = Self::youtube_new(reqwest_client, &song, user_name).await;

        if let Ok(songs) = res_new {
            return Ok(songs);
        }

        #[cfg(feature = "yt-dlp-fallback")]
        if let Ok(res_old) = Self::youtube_old(&song, user_name).await {
            log!(
                warn,
                "new scrapper failed, falling back to yt-dlp";
                "{}", (res_new.err().expect("Its already an error"))
            );
            return Ok(res_old);
        }

        // TODO: better error menagement
        res_new
    }

    /// Sends GET request to `YouTube` as if it was requested from a browser and
    /// scrapes the result.
    async fn youtube_new(
        reqwest_client: &Client,
        song: &str,
        user_name: &str,
    ) -> Result<VecDeque<Self>> {
        let res = reqwest_client.get(song).send().await?.text().await?;

        let mut song_list = VecDeque::new();

        if song.contains("/watch?") {
            if song.contains("&list=") {
                let yt_initial_data = &res[res
                    .find("ytInitialData")
                    .ok_or_else(|| anyhow!("Parse error"))?
                    + "ytInitialData = ".len() ..];
                let yt_initial_data = &yt_initial_data[.. yt_initial_data
                    .find("</script>")
                    .ok_or_else(|| anyhow!("Parse error"))?
                    - ";".len()];

                song_list.extend(
                    sonic_rs::from_str::<YoutubeVideoPlaylist>(yt_initial_data)?
                        .contents
                        .two_column_watch_next_results
                        .playlist
                        .playlist
                        .contents
                        .into_iter()
                        .map(|video| {
                            Self {
                                title:     video.playlist_panel_video_renderer.title.simple_text,
                                id:        video
                                    .playlist_panel_video_renderer
                                    .navigation_endpoint
                                    .watch_endpoint
                                    .video_id,
                                duration:  video
                                    .playlist_panel_video_renderer
                                    .length_text
                                    .simple_text,
                                user_name: user_name.to_owned(),
                            }
                        }),
                );
            } else {
                let yt_initial_player_response = &res[res
                    .find("ytInitialPlayerResponse")
                    .ok_or_else(|| anyhow!("Parse error"))?
                    + "ytInitialPlayerResponse = ".len() ..];
                let yt_initial_player_response = &yt_initial_player_response
                    [.. yt_initial_player_response
                        .find(";var")
                        .ok_or_else(|| anyhow!("Parse error"))?];

                let video_details =
                    sonic_rs::from_str::<YoutubeVideo>(yt_initial_player_response)?.video_details;

                song_list.push_back(Self {
                    title:     video_details.title,
                    id:        video_details.video_id,
                    duration:  video_details.length_seconds,
                    user_name: user_name.to_owned(),
                });
            }

            return Ok(song_list);
        } else if song.contains("/playlist?") {
            let yt_initial_data = &res[res
                .find("ytInitialData")
                .ok_or_else(|| anyhow!("Parse error"))?
                + "ytInitialData = ".len() ..];
            let yt_initial_data = &yt_initial_data[.. yt_initial_data
                .find("</script>")
                .ok_or_else(|| anyhow!("Parse error"))?
                - ";".len()];

            song_list.extend(
                sonic_rs::from_str::<YoutubePlaylist>(yt_initial_data)?
                    .contents
                    .two_column_browse_results_renderer
                    .tabs
                    .into_iter()
                    .filter_map(|tab| tab.tab_renderer)
                    .flat_map(|tab_renderer| tab_renderer.content.section_list_renderer.contents)
                    .filter_map(|contents| contents.item_section_renderer)
                    .flat_map(|renderer| renderer.contents)
                    .filter_map(|content| content.playlist_video_list_renderer)
                    .flat_map(|renderer| renderer.contents)
                    .map(|mut video| {
                        Self {
                            title:     std::mem::take(
                                &mut video.playlist_video_renderer.title.runs[0].text,
                            ),
                            id:        video.playlist_video_renderer.video_id,
                            duration:  video.playlist_video_renderer.length_text.simple_text,
                            user_name: user_name.to_owned(),
                        }
                    }),
            );

            return Ok(song_list);
        }

        Err(anyhow!("Unsupported YouTube link type"))
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
            log!(error, "YouTube data fetch with yt-dlp failed:"; "{}", (String::from_utf8_lossy(&res.stderr)));
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
    pub async fn spotify(
        ctx: &Context<'_>,
        reqwest_client: &Client,
        song: String,
        user_name: &str,
    ) -> Result<VecDeque<Self>> {
        let Some(token) = get_config!().spotify_token().await else {
            return Err(anyhow!("Spotify token is not initialized"));
        };

        let (url_type, id, extra) = match *song.split('/').take(5).collect::<Vec<_>>().as_slice() {
            ["https:", "", "open.spotify.com", "track", last] => ("tracks", get_id!(last), ""),
            ["https:", "", "open.spotify.com", "playlist", last] =>
                ("playlists", get_id!(last), ""),
            ["https:", "", "open.spotify.com", "album", last] => ("albums", get_id!(last), ""),
            ["https:", "", "open.spotify.com", "artist", last] =>
                ("artists", get_id!(last), "/top-tracks"),
            _ => return Err(anyhow!("Unsupported Spotify URL type")),
        };

        let Ok(res) = reqwest_client
            .get(format!("{SP_BASE_URL}/{url_type}/{id}{extra}"))
            .bearer_auth(token)
            .query(&[("market", SP_MARKET)])
            .send()
            .await
        else {
            return Err(anyhow!("Couldn't connect to Spotify"));
        };

        if !res.status().is_success() {
            log!(error, "Spotify data fetch failed:"; "{}", (sonic_rs::from_str::<SpotifyError>(&res.text().await?)?.message));
            return Err(anyhow!("Couldn't connect to Spotify"));
        }

        let res = res.text().await?;

        let list = match url_type {
            "tracks" =>
                vec![sonic_rs::from_str::<SpotifyTrack>(&res).map(|mut track| {
                    (
                        std::mem::take(&mut track.artists[0].name),
                        track.name,
                    )
                })?],
            "playlists" =>
                sonic_rs::from_str::<SpotifyPlaylist>(&res)?
                    .tracks
                    .items
                    .into_iter()
                    .map(|mut track| {
                        (
                            std::mem::take(&mut track.track.artists[0].name),
                            track.track.name,
                        )
                    })
                    .collect::<Vec<_>>(),
            "albums" =>
                sonic_rs::from_str::<SpotifyAlbum>(&res)?
                    .tracks
                    .items
                    .into_iter()
                    .map(|mut track| {
                        (
                            std::mem::take(&mut track.artists[0].name),
                            track.name,
                        )
                    })
                    .collect::<Vec<_>>(),
            "artists" =>
                sonic_rs::from_str::<SpotifyArtistTopTracks>(&res)?
                    .tracks
                    .into_iter()
                    .map(|mut track| {
                        (
                            std::mem::take(&mut track.artists[0].name),
                            track.name,
                        )
                    })
                    .collect::<Vec<_>>(),
            _ => unreachable!("url_type cannot be anything else"),
        };

        let mut skipped_count = 0;
        let songs = join_all(list.into_iter().map(|(artist, song)| {
            Self::search(
                ctx,
                reqwest_client,
                format!("{} - {} lyrics", artist, song),
                user_name,
                1,
            )
        }))
        .await
        .into_iter()
        .filter_map(|res| {
            if res.is_err() {
                skipped_count += 1;
                return None;
            }

            let songs = res.expect("Cannot be error");
            if songs.is_empty() {
                skipped_count += 1;
                return None;
            }

            Some(songs)
        })
        .flatten()
        .collect::<VecDeque<_>>();

        if skipped_count > 0 {
            message!(error, ctx, ("{} songs skipped", skipped_count); true);
        }

        Ok(songs)
    }

    /// gets [`songbird::input::Input`] for music stream
    pub async fn get_input(&self, reqwest_client: &Client) -> Result<Input> {
        let res_new = self.get_input_new(reqwest_client).await;

        if let Ok(input) = res_new {
            return Ok(input);
        }

        #[cfg(feature = "yt-dlp-fallback")]
        {
            log!(
                warn,
                "new scrapper failed as input generation, falling back to yt-dlp";
                "{}", (res_new.err().expect("Its already an error"))
            );

            Ok(self.get_input_old(reqwest_client))
        }

        #[cfg(not(feature = "yt-dlp-fallback"))]
        res_new
    }

    /// Sends GET request to `YouTube` as if it was searched in browser and
    /// scrapes the results.
    async fn get_input_new(&self, reqwest_client: &Client) -> Result<Input> {
        let res = reqwest_client
            .get(format!(
                "https://www.youtube.com/watch?v={}",
                self.id
            ))
            .send()
            .await?
            .text()
            .await?;

        let yt_initial_player_response = &res[res
            .find("ytInitialPlayerResponse")
            .ok_or_else(|| anyhow!("Parse error"))?
            + "ytInitialPlayerResponse = ".len() ..];
        let yt_initial_player_response = &yt_initial_player_response[.. yt_initial_player_response
            .find(";var")
            .ok_or_else(|| anyhow!("Parse error"))?];

        let streaming_data =
            sonic_rs::from_str::<YoutubePlayer>(yt_initial_player_response)?.streaming_data;
        let (mut audio_formats, mut video_formats) =
            [streaming_data.formats, streaming_data.adaptive_formats]
                .into_iter()
                .flatten()
                .fold(
                    (Vec::new(), Vec::new()),
                    |(mut audio, mut video), format| {
                        if format.mime_type.contains("audio/") {
                            audio.push(format);
                        } else {
                            video.push(format);
                        }

                        (audio, video)
                    },
                );

        let selected_format = if !audio_formats.is_empty() {
            audio_formats.sort_by(|a, b| a.bitrate.cmp(&b.bitrate));

            // get best bitrate
            audio_formats
                .pop()
                .expect("Allready check for at least one element")
        } else {
            video_formats.sort_by(|a, b| b.bitrate.cmp(&a.bitrate));

            // get worst bitrate
            video_formats
                .pop()
                .expect("Always has at least one element")
        };

        // let (client, url) = self.url_extractor(selected_format)?;
        let (url, headers) = Self::url_extractor(selected_format)?;

        Ok(HttpRequest::new_with_headers(reqwest_client.clone(), url, headers).into())
    }

    /// Solves the `signature_cipher` for getting the URL
    ///
    /// `YouTube` changes `signature_cipher` logic very frequently; so, this has
    /// very high possibility to fail in the future.
    fn url_extractor(format: Format) -> Result<(String, HeaderMap)> {
        let mut headers = HeaderMap::with_capacity(6);
        headers.insert(
            HeaderName::from_static("user-agent"),
            USER_AGENT.parse()?,
        );
        headers.insert(
            HeaderName::from_static("accept"),
            "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8".parse()?,
        );
        headers.insert(
            HeaderName::from_static("accept-language"),
            "en-us,en;q=0.5".parse()?,
        );
        headers.insert(
            HeaderName::from_static("sec-fetch-mode"),
            "navigate".parse()?,
        );
        headers.insert(
            HeaderName::from_static("origin"),
            "https://www.youtube.com".parse()?,
        );
        headers.insert(
            HeaderName::from_static("referer"),
            "https://www.youtube.com/".parse()?,
        );

        if let Some(signature_cipher) = format.signature_cipher {
            let (_s, _sp, _url) = signature_cipher
                .split('\u{0026}')
                .map(|part| part.split_once('=').expect("Always has '='"))
                .fold(
                    (String::new(), String::new(), String::new()),
                    |(mut s, mut sp, mut url), (key, value)| {
                        match key {
                            "s" => s.push_str(value),
                            "sp" => sp.push_str(value),
                            "url" => url.push_str(value),
                            _ => (),
                        }

                        (s, sp, url)
                    },
                );

            Err(anyhow!("URL extractor is incomplete"))
        } else if let Some(url) = format.url {
            println!("URL: {}", url);
            // FIXME: somehow does not work
            Ok((url, headers))
        } else {
            Err(anyhow!("No source found"))
        }
    }

    /// Uses old `yt-dlp` to get the song stream.
    #[cfg(feature = "yt-dlp-fallback")]
    fn get_input_old(&self, reqwest_client: &Client) -> Input {
        YoutubeDl::new(reqwest_client.clone(), self.id.clone()).into()
    }

    /// Get title of the song.
    pub fn title(&self) -> &str { &self.title }

    /// Get `YouTube` URL of the song.
    pub fn id(&self) -> &str { &self.id }

    /// Get duration of the song.
    pub fn duration(&self) -> &str { &self.duration }

    /// Get `Discord` user name of the person who requested the song.
    pub fn user_name(&self) -> &str { &self.user_name }
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
