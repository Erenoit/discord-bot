use crate::{bot::Context, get_config, logger, messager, player::sp_structs::*};
use std::{collections::VecDeque, fmt::Display};
use anyhow::{anyhow, Result};
use tokio::{process::Command, task::JoinSet};

const SP_MARKET: &str = "US";

#[derive(Clone)]
pub struct Song {
    title: String,
    id: String,
    duration: String,
    user_name: String,
}

impl Song {
    pub async fn new<S: Display + Send>(ctx: &Context<'_>, song: S) -> Result<VecDeque<Self>> {
        let song = song.to_string();
        let user_name = ctx.author().name.to_string();

        if song.starts_with("http://") || song.starts_with("https://") {
            if song.contains("youtube") {
                if song.contains("list") {
                    Self::yt_url(song, user_name).await
                } else {
                    Self::yt_playlist(song, user_name).await
                }
            } else if song.contains("spotify") {
                if song.contains("/track/") {
                    Self::sp_url(song, user_name).await
                } else if song.contains("/playlist/") {
                    Self::sp_playlist(song, user_name).await
                } else if song.contains("/artist/") {
                    Self::sp_artist(song, user_name).await
                } else if song.contains("/album/") {
                    Self::sp_album(song, user_name).await
                } else {
                    messager::send_error(ctx, "Unsupported spotify url", true).await;
                    Err(anyhow!("Unsupported spotify url"))
                }
            } else {
                messager::send_error(ctx, "Unsupported music source", true).await;
                Err(anyhow!("Unsupported music source"))
            }
        } else {
            Self::yt_search(ctx, song, user_name).await
        }
    }

    #[inline(always)]
    pub async fn yt_search(ctx: &Context<'_>, song: String, user_name: String) -> Result<VecDeque<Self>>
    {
        // TODO: change something faster than youtube-dl
        // TODO: clean this code
        let search_count = 5;
        let out = Command::new("youtube-dl")
            .args(["--no-playlist", "--get-title", "--get-id", "--get-duration", &format!("ytsearch{search_count}:{song}")])
            .output().await?;


        let list = String::from_utf8(out.stdout).unwrap();
        let list_seperated = list.split('\n').collect::<Vec<_>>();

        let mut l: Vec<(String, String)> = Vec::with_capacity(search_count);
        for i in 0 .. search_count {
            l.push((list_seperated[i * 3].to_string(), list_seperated[i * 3 + 1].to_string()));
        }

        let answer = messager::send_selection_from_list(ctx, "Search", &l).await;
        if answer == "success" {
            let mut return_vec = VecDeque::with_capacity(search_count);
            for i in 0 .. search_count {
                return_vec.push_back(Self {
                    title:     list_seperated[i * 3].to_string(),
                    id:        list_seperated[i * 3].to_string(),
                    duration:  list_seperated[i * 3].to_string(),
                    user_name: user_name.to_string(),
                });
            }
            Ok(return_vec)
        } else if answer != "danger" {
            let mut return_vec = VecDeque::with_capacity(1);
            for i in 0 .. search_count {
                if *list_seperated[i * 3 + 1] == answer {
                    return_vec.push_back(Self {
                        title:     list_seperated[i * 3].to_string(),
                        id:        list_seperated[i * 3 + 1].to_string(),
                        duration:  list_seperated[i * 3 + 2].to_string(),
                        user_name: user_name.to_string(),
                    });
                    break;
                }
            }
            Ok(return_vec)
        } else {
            Err(anyhow!("Selection failed/canceled"))
        }
    }

    // TODO: youtube-dl is slow sometimes
    // TODO: cannot open age restricted videos
    #[inline(always)]
    async fn yt_url(song: String, user_name: String) -> Result<VecDeque<Self>> {
        if let Ok(res) = Command::new("youtube-dl")
            .args(["--get-title", "--get-id", "--get-duration", &song])
            .output().await
            {
                if !res.status.success() {
                    logger::error("YouTube data fetch with youtube-dl failed:");
                    logger::secondary_error(String::from_utf8(res.stderr).expect("Output must be valid UTF-8"));
                    return Err(anyhow!("youtube-dl failed"));
                }

                let splited_res: Vec<String> = String::from_utf8(res.stdout)
                    .expect("Output must be valid UTF-8")
                    .split('\n')
                    .map(|e| e.to_string())
                    .collect();

                let title    = splited_res.get(0);
                let id       = splited_res.get(1);
                let duration = splited_res.get(2);

                if title.is_none() || id.is_none() || duration.is_none() {
                    logger::error("Somehow youtube-dl returned less data");
                    return Err(anyhow!("youtube-dl failed"));
                }

                let mut return_vec = VecDeque::with_capacity(1);
                return_vec.push_back(Self {
                    title:    title.unwrap().to_string(),
                    id:       id.unwrap().to_string(),
                    duration: duration.unwrap().to_string(),
                    user_name,
                });
                Ok(return_vec)
            } else {
                logger::error("Command creation for youtube-dl failed");
                Err(anyhow!("youtube-dl failed"))
            }
    }

    #[inline(always)]
    async fn yt_playlist(song: String, user_name: String) -> Result<VecDeque<Self>> {
        if let Ok(res) = Command::new("youtube-dl")
            .args(["--flat-playlist","--get-title", "--get-id", "--get-duration", &song])
            .output().await
            {
                if !res.status.success() {
                    logger::error("YouTube data fetch with youtube-dl failed:");
                    logger::secondary_error(String::from_utf8(res.stderr).expect("Output must be valid UTF-8"));
                    return Err(anyhow!("youtube-dl failed"));
                }

                let splited_res: Vec<String> = String::from_utf8(res.stdout)
                    .expect("Output must be valid UTF-8")
                    .split('\n')
                    .filter(|e| !e.is_empty())
                    .map(|e| e.to_string())
                    .collect();

                if splited_res.len() % 3 != 0 {
                    logger::error("youtube-dl returned wrong number of arguments");
                    return Err(anyhow!("Output must be dividable by 3"))
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
                logger::error("Command creation for youtube-dl failed");
                Err(anyhow!("youtube-dl failed"))
            }
    }

    #[inline(always)]
    async fn sp_url(song: String, user_name: String) -> Result<VecDeque<Self>> {
        let base_url = "https://api.spotify.com/v1";
        let track_id = song.split("/track/").collect::<Vec<_>>()[1].split('?').collect::<Vec<_>>()[0];
        let token = get_config().spotify_token().await.expect("Token should be initialized");

        let res = reqwest::Client::new()
            .get(format!("{base_url}/tracks/{track_id}"))
            .bearer_auth(token)
            .send()
            .await;

        match res {
            Ok(r) => {
                let j = json::parse(&r.text().await?)?;
                let title = &j["name"];

                let out = Command::new("youtube-dl")
                    .args(["--no-playlist", "--get-title", "--get-id", &format!("ytsearch:{title} lyrics")])
                    .output().await?;

                let out_str = String::from_utf8(out.stdout)?;
                let mut song = out_str.split('\n');

                let title = song.next();
                let id = song.next();
                let duration = song.next();

                if title.is_some() && id.is_some() && duration.is_some() {
                    let mut return_vec = VecDeque::with_capacity(1);
                    return_vec.push_back(Self {
                        title:    title.unwrap().to_string(),
                        id:       id.unwrap().to_string(),
                        duration: duration.unwrap().to_string(),
                        user_name
                    });
                    Ok(return_vec)
                } else {
                    Err(anyhow!("coudn't find the song on youtube"))
                }
            }
            Err(why) => {
                logger::error("Spotify request failed");
                logger::secondary_error(why);

                Err(anyhow!("Spotify request failed"))
            }
        }
    }

    #[inline(always)]
    async fn sp_playlist(song: String, user_name: String) -> Result<VecDeque<Self>> {
        let base_url = "https://api.spotify.com/v1";
        let track_id = song.split("/playlist/").collect::<Vec<_>>()[1].split('?').collect::<Vec<_>>()[0];
        let token = get_config().spotify_token().await.expect("Token should be initialized");

        let res = reqwest::Client::new()
            .get(format!("{base_url}/playlists/{track_id}"))
            .bearer_auth(token)
            .send()
            .await;

        match res {
            Ok(r) => {
                let j = r.json::<SpotifyPlaylistResponse>().await?;

                let mut join_set = JoinSet::new();

                j.tracks.items.iter()
                    .for_each(|track| {
                        let title = track.track.name.clone();

                        join_set.spawn(async move {Command::new("youtube-dl")
                            .args(["--no-playlist", "--get-title", "--get-id", &format!("ytsearch:{title} lyrics")])
                            .output().await});
                    });

                let mut tracklist = VecDeque::with_capacity(j.tracks.items.len());

                while let Some(Ok(Ok(track))) = join_set.join_next().await {
                    let res = String::from_utf8(track.stdout).unwrap();
                    let mut res_split = res.split('\n');

                    tracklist.push_back(Self {
                        title: res_split.next().unwrap().to_string(),
                        id: res_split.next().unwrap().to_string(),
                        duration: res_split.next().unwrap().to_string(),
                        user_name: user_name.clone(),
                    });
                }

                Ok(tracklist)
            }
            Err(why) => {
                logger::error("Spotify request failed");
                logger::secondary_error(why);

                Err(anyhow!("Spotify request failed"))
            }
        }
    }

    #[inline(always)]
    async fn sp_artist(song: String, user_name: String) -> Result<VecDeque<Self>> {
        let base_url = "https://api.spotify.com/v1";
        let track_id = song.split("/artist/").collect::<Vec<_>>()[1].split('?').collect::<Vec<_>>()[0];
        let token = get_config().spotify_token().await.expect("Token should be initialized");

        let res = reqwest::Client::new()
            .get(format!("{base_url}/artists/{track_id}/top-tracks"))
            .bearer_auth(token)
            .query(&[("market", SP_MARKET)])
            .send()
            .await;

        match res {
            Ok(r) => {
                let j = r.json::<SpotifyArtistTopTracksResponse>().await?;

                let mut join_set = JoinSet::new();

                j.tracks.iter()
                    .for_each(|track| {
                        let title = track.name.clone();

                        join_set.spawn(async move {Command::new("youtube-dl")
                            .args(["--no-playlist", "--get-title", "--get-id", &format!("ytsearch:{title} lyrics")])
                            .output().await});
                    });

                let mut tracklist = VecDeque::with_capacity(j.tracks.len());

                while let Some(Ok(Ok(track))) = join_set.join_next().await {
                    let res = String::from_utf8(track.stdout).unwrap();
                    let mut res_split = res.split('\n');

                    tracklist.push_back(Self {
                        title: res_split.next().unwrap().to_string(),
                        id: res_split.next().unwrap().to_string(),
                        duration: res_split.next().unwrap().to_string(),
                        user_name: user_name.clone(),
                    });
                }

                Ok(tracklist)
            }
            Err(why) => {
                logger::error("Spotify request failed");
                logger::secondary_error(why);

                Err(anyhow!("Spotify request failed"))
            }
        }
    }

    #[inline(always)]
    async fn sp_album(song: String, user_name: String) -> Result<VecDeque<Self>> {
        let base_url = "https://api.spotify.com/v1";
        let track_id = song.split("/album/").collect::<Vec<_>>()[1].split('?').collect::<Vec<_>>()[0];
        let token = get_config().spotify_token().await.expect("Token should be initialized");

        let res = reqwest::Client::new()
            .get(format!("{base_url}/albums/{track_id}"))
            .bearer_auth(token)
            .send()
            .await;

        match res {
            Ok(r) => {
                let j = r.json::<SpotifyAlbumResponse>().await?;

                let mut join_set = JoinSet::new();

                j.tracks.items.iter()
                    .for_each(|track| {
                        let title = track.name.clone();

                        join_set.spawn(async move {Command::new("youtube-dl")
                            .args(["--no-playlist", "--get-title", "--get-id", &format!("ytsearch:{title} lyrics")])
                            .output().await});
                    });

                let mut tracklist = VecDeque::with_capacity(j.tracks.items.len());

                while let Some(Ok(Ok(track))) = join_set.join_next().await {
                    let res = String::from_utf8(track.stdout).unwrap();
                    let mut res_split = res.split('\n');

                    tracklist.push_back(Self {
                        title: res_split.next().unwrap().to_string(),
                        id: res_split.next().unwrap().to_string(),
                        duration: res_split.next().unwrap().to_string(),
                        user_name: user_name.clone(),
                    });
                }

                Ok(tracklist)
            }
            Err(why) => {
                logger::error("Spotify request failed");
                logger::secondary_error(why);

                Err(anyhow!("Spotify request failed"))
            }
        }
    }

    #[inline(always)]
    pub fn title(&self) -> String {
        self.title.clone()
    }

    #[inline(always)]
    pub fn url(&self) -> String {
        self.id.clone()
    }

    #[inline(always)]
    pub fn duration(&self) -> String {
        self.duration.clone()
    }

    #[inline(always)]
    pub fn user_name(&self) -> String {
        self.user_name.clone()
    }
}

impl Display for Song {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}] {}", self.title(), self.duration(), messager::highlight(format!("requested by {}", self.user_name())))
    }
}

