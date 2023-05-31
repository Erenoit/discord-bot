mod event;
mod song;
#[cfg(feature = "spotify")]
mod sp_structs;

use std::{collections::VecDeque, fmt::Write, mem, slice::Iter};

use serenity::model::id::{ChannelId, GuildId};
use songbird::{Event, TrackEvent};
use tokio::sync::Mutex;

pub use crate::player::song::Song;
use crate::{bot::Context, player::event::SongEnd};

macro_rules! get_songbird_manager {
    () => {
        get_config!().songbird()
    };
}

macro_rules! get_call_mutex {
    ($($guild_id:tt)+) => {
        get_songbird_manager!().get($($guild_id)+)
    };
}

#[non_exhaustive]
pub struct Player {
    guild_id:     GuildId,
    now_playing:  Mutex<Option<Song>>,
    repeat_mode:  Mutex<Repeat>,
    song_queue:   Mutex<VecDeque<Song>>,
    repeat_queue: Mutex<VecDeque<Song>>,
}

impl Player {
    pub fn new(guild_id: GuildId) -> Self {
        Self {
            guild_id,
            now_playing: Mutex::new(None),
            repeat_mode: Mutex::new(Repeat::Off),
            song_queue: Mutex::new(VecDeque::with_capacity(100)),
            repeat_queue: Mutex::new(VecDeque::with_capacity(100)),
        }
    }

    pub async fn connect_to_voice_channel(&self, channel_id: &ChannelId) {
        let (call_mutex, result) = get_songbird_manager!()
            .join(self.guild_id, *channel_id)
            .await;

        if let Err(why) = result {
            log!(error, "Couldn't join the voice channel."; "{why}");
        } else {
            let mut call = call_mutex.lock().await;
            if let Err(why) = call.deafen(true).await {
                log!(error, "Couldn't deafen the bot."; "{why}");
            }
        }
    }

    pub async fn leave_voice_channel(&self, ctx: &Context<'_>) {
        if self.connected_vc().await.is_none() {
            message!(error, ctx, ("Not in a voice channel"); true);
            return;
        }

        if let Some(call_mutex) = get_call_mutex!(self.guild_id) {
            let mut call = call_mutex.lock().await;

            call.leave()
                .await
                .expect("There should be no error while leaving the call");
        }
    }

    pub async fn play(&self, songs: &mut VecDeque<Song>) {
        self.song_queue.lock().await.append(songs);

        if self.now_playing.lock().await.is_none() {
            self.start_stream().await;
        }
    }

    pub async fn start_stream(&self) {
        let repeat_mode = self.get_repeat_mode().await;
        let Some(call_mutex) = get_call_mutex!(self.guild_id) else {
            unreachable!("Not in a voice channel to play in")
        };

        match repeat_mode {
            Repeat::Off =>
                if self.song_queue.lock().await.is_empty() {
                    self.stop_stream().await;
                    return;
                },
            Repeat::One => {},
            Repeat::All =>
                if self.song_queue.lock().await.is_empty() {
                    mem::swap(
                        &mut *self.song_queue.lock().await,
                        &mut *self.repeat_queue.lock().await,
                    );
                },
        }

        let next_song = match repeat_mode {
            Repeat::Off | Repeat::All =>
                self.song_queue
                    .lock()
                    .await
                    .pop_front()
                    .expect("Queue cannot be empty at this point"),
            Repeat::One => {
                let mut now_playing = self.now_playing.lock().await;
                if now_playing.is_some() {
                    (*now_playing).take().expect("Cannot be None at this point")
                } else {
                    let Some(song) = self.song_queue
                        .lock()
                        .await
                        .pop_front() else {
                            self.stop_stream().await;
                            return;
                        };
                    song
                }
            },
        };

        let source = match songbird::ytdl(next_song.url()).await {
            Ok(source) => source,
            Err(why) => {
                log!(error, "Couldn't start source."; "{why}");
                return;
            },
        };

        let mut call = call_mutex.lock().await;
        _ = call
            .play_source(source)
            .add_event(Event::Track(TrackEvent::End), SongEnd {
                guild_id: self.guild_id,
            });
        *self.now_playing.lock().await = Some(next_song);
    }

    pub async fn stop_stream(&self) {
        if let Some(call_mutex) = get_call_mutex!(self.guild_id) {
            let mut call = call_mutex.lock().await;
            call.stop();
            *self.now_playing.lock().await = None;
        }
    }

    pub async fn skip_song(&self) {
        self.move_to_repeat_queue().await;
        self.stop_stream().await;
        if self.get_repeat_mode().await == Repeat::One {
            *self.repeat_mode.lock().await = Repeat::Off;
            self.start_stream().await;
            *self.repeat_mode.lock().await = Repeat::One;
        } else {
            self.start_stream().await;
        }
    }

    pub async fn move_to_repeat_queue(&self) {
        if self.now_playing.lock().await.is_some() {
            self.repeat_queue
                .lock()
                .await
                .push_back(self.now_playing.lock().await.as_ref().unwrap().clone());
        }
    }

    pub async fn clear_the_queues(&self) {
        mem::take(&mut *self.song_queue.lock().await);
        mem::take(&mut *self.repeat_queue.lock().await);
    }

    pub async fn shuffle_song_queue(&self) {
        let mut queue = self.song_queue.lock().await;
        #[allow(clippy::significant_drop_in_scrutinee)]
        for i in 0 ..= queue.len() - 2 {
            let j = rand::random::<usize>() % (queue.len() - i) + i;
            queue.swap(i, j);
        }
    }

    #[allow(clippy::significant_drop_tightening)]
    pub async fn print_queue(&self, ctx: &Context<'_>) {
        if self.now_playing.lock().await.is_none() {
            message!(error, ctx, ("Nothings playing :unamused:"); true);
            return;
        }

        let mut s = String::with_capacity(1024);
        let s_queue = self.song_queue.lock().await;
        let r_queue = self.repeat_queue.lock().await;
        let s_len = s_queue.len();
        let r_len = r_queue.len();
        let (after, before) = {
            let is_song_queue_enough = s_len >= 5;
            let is_repeat_queue_enough = r_len >= 5;

            if !is_song_queue_enough && !is_repeat_queue_enough {
                (s_len, r_len)
            } else if !is_song_queue_enough {
                (s_len, std::cmp::min(10 - s_len, r_len))
            } else if !is_repeat_queue_enough {
                (std::cmp::min(10 - r_len, s_len), r_len)
            } else {
                (5, 5)
            }
        };

        let mut num = r_len - before + 1;

        for i in (r_len - before) .. r_len {
            Self::add_to_queue_string(&mut s, &r_queue[i], num, false);
            num += 1;
        }

        Self::add_to_queue_string(
            &mut s,
            self.now_playing.lock().await.as_ref().unwrap(),
            num,
            true,
        );
        num += 1;

        for i in 0 .. after {
            Self::add_to_queue_string(&mut s, &s_queue[i], num, false);
            num += 1;
        }

        message!(normal, ctx, ("Queue"); ("{}", s); false);
    }

    fn add_to_queue_string(s: &mut String, song: &Song, num: usize, selected: bool) {
        let selected_char = "➤";
        let selected_whitespace = "  ";
        let normal_whitespace = "     ";
        let number_style = format!("{num}) ");
        let song_str = song.to_string();

        if selected {
            _ = writeln!(
                s,
                "**{selected_char}{selected_whitespace}{number_style}{song_str}**"
            );
        } else {
            _ = writeln!(s, "{normal_whitespace}{number_style}{song_str}");
        }
    }

    pub async fn is_queues_empty(&self) -> bool {
        self.song_queue.lock().await.is_empty() && self.repeat_queue.lock().await.is_empty()
    }

    pub async fn change_repeat_mode(&self, ctx: &Context<'_>, new_mode: &Repeat) {
        *self.repeat_mode.lock().await = *new_mode;
        message!(success, ctx, ("Repeat mode changed to {new_mode}"); false);
    }

    pub async fn get_repeat_mode(&self) -> Repeat { *self.repeat_mode.lock().await }

    pub async fn connected_vc(&self) -> Option<songbird::id::ChannelId> {
        if let Some(call_mutex) = get_call_mutex!(self.guild_id) {
            call_mutex.lock().await.current_channel()
        } else {
            None
        }
    }
}

#[derive(poise::ChoiceParameter, Copy, Clone, Eq, PartialEq)]
pub enum Repeat {
    Off,
    One,
    All,
}

impl Repeat {
    pub fn variants() -> Iter<'static, Self> {
        static V: [Repeat; 3] = [Repeat::Off, Repeat::One, Repeat::All];
        V.iter()
    }
}
