mod song;

use crate::{bot::commands::Context, CONFIG, logger, messager, player::song::Song};
use std::{collections::VecDeque, sync::Arc};
use serenity::model::id::{ChannelId, GuildId};
use songbird::{Call, Songbird};
use tokio::sync::Mutex;

#[inline(always)]
fn get_songbird_manager() -> Arc<Songbird> {
    CONFIG.get().unwrap().songbird()
}

#[inline(always)]
fn get_call_mutex(guild_id: GuildId) -> Option<Arc<Mutex<Call>>> {
    get_songbird_manager().get(guild_id)
}

#[inline(always)]
async fn is_in_vc(guild_id: GuildId) -> bool {
    if let Some(call_mutex) = get_call_mutex(guild_id) {
        call_mutex.lock().await.current_channel().is_some()
    } else {
        false
    }
}

#[inline(always)]
fn context_to_voice_channel_id(ctx: &Context<'_>) -> Option<ChannelId> {
    ctx.guild().expect("Guild should be Some")
                .voice_states.get(&ctx.author().id)
                .and_then(|voice_state| voice_state.channel_id)
}

pub struct Player {
    guild_id: GuildId,
    now_playing:  Mutex<Option<Song>>,
    repeat_mode:  Mutex<Repeat>,
    song_queue:   Mutex<VecDeque<Song>>,
    repeat_queue: Mutex<VecDeque<Song>>,
}

impl Player {
    pub fn new(guild_id: GuildId) -> Self {
        Self {
            guild_id,
            now_playing:  Mutex::new(None),
            repeat_mode:  Mutex::new(Repeat::Off),
            song_queue:   Mutex::new(VecDeque::with_capacity(100)),
            repeat_queue: Mutex::new(VecDeque::with_capacity(100)),
        }
    }

    pub async fn connect_to_voice_channel(&self, channel_id: &ChannelId) {
        let manager = get_songbird_manager();

        let (call_mutex, result) = manager.join(self.guild_id, *channel_id).await;

        if let Err(why) = result {
            logger::error("Couldn't join the voice channel.");
            logger::secondary_error(why);
        } else {
            let mut call = call_mutex.lock().await;
            if let Err(why) = call.deafen(true).await {
                logger::error("Couldn't deafen the bot.");
                logger::secondary_error(why);
            }
        }
    }

    pub async fn leave_voice_channel(&self, ctx: &Context<'_>) {
        if !is_in_vc(self.guild_id).await {
            messager::send_error(ctx, "Not in a voice channel", true).await;
            return;
        }

        if let Some(call_mutex) = get_call_mutex(self.guild_id) {
            let mut call = call_mutex.lock().await;

            call.leave().await.expect("There shold be no error while leaving the call");
        }
    }

    pub async fn play(&self, ctx: &Context<'_>, url: String) {
        if !is_in_vc(self.guild_id).await {
            if let Some(channel_id) = context_to_voice_channel_id(ctx) {
                self.connect_to_voice_channel(&channel_id).await;
            } else {
                messager::send_error(ctx, "You are not in the voice channel", true).await;
                return;
            }
        }

        if url.contains("list=") || url.contains("/playlist/") {
            if let Ok(list) = Song::from_playlist(url, &ctx.author().name).await {
                // CHECK: if "Vec -> VecDeque" reallocates the memmory
                messager::send_sucsess(ctx, format!("{} songs added to the list", list.len()), true).await;
                self.song_queue.lock().await.append(&mut VecDeque::from(list));
            } else {
                messager::send_error(ctx, "Error happened while fetching data about playlist. Please try again later.", true).await;
                return;
            }
        } else if let Ok(s) = Song::new(url, &ctx.author().name).await {
            messager::send_sucsess(ctx, format!("{} is added to the list", s.title()), true).await;
            self.song_queue.lock().await.push_back(s);
        } else {
            messager::send_error(ctx, "Error happened while fetching data about song. Please try again later.", true).await;
            return;
        }

        if self.now_playing.lock().await.is_none() {
            self.start_stream().await
        }
    }

    pub async fn start_stream(&self) {
        if self.song_queue.lock().await.is_empty() { self.stop_stream().await; return; }

        if let Some(call_mutex) = get_call_mutex(self.guild_id) {
            let next_song = self.song_queue.lock().await.pop_front().expect("Queue cannot be empty at this point");

            let source = match songbird::ytdl(next_song.url()).await {
                Ok(source) => source,
                Err(why) => {
                    logger::error("Couldn't start source.");
                    logger::secondary_error(why);
                    return;
                },
            };

            let mut call = call_mutex.lock().await;
            call.play_source(source);
            *self.now_playing.lock().await  = Some(next_song);
        } else {
            unreachable!("Not in a voice channel to play in")
        }
    }

    pub async fn stop_stream(&self) {
        if let Some(call_mutex) = get_call_mutex(self.guild_id) {
            let mut call = call_mutex.lock().await;
            call.stop();
            *self.now_playing.lock().await = None;
        }
    }

    pub async fn skip_song(&self) {
        self.move_to_repeat_queue().await;
        self.stop_stream().await;
        self.start_stream().await;
    }

    pub async fn move_to_repeat_queue(&self) {
        if self.now_playing.lock().await.is_some() {
            self.repeat_queue.lock().await.push_back(self.now_playing.lock().await.as_ref().unwrap().clone());
        }
    }

    pub async fn clear_the_queues(&self) {
        *self.song_queue.lock().await   = VecDeque::with_capacity(100);
        *self.repeat_queue.lock().await = VecDeque::with_capacity(100);
    }

    pub async fn shuffle_song_queue(&self) {
        let mut queue = self.song_queue.lock().await;
        for i in 0 ..= queue.len() - 2 {
          let j = (rand::random::<f32>() * (i as f32 - 1.0)) as usize;
          queue.swap(i, j);
        }
    }

    pub async fn print_queue(&self, ctx: &Context<'_>) {
        if self.now_playing.lock().await.is_none() {
            messager::send_error(ctx, "Nothings playing :unamused:", true).await;
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
            num +=1;
        }

        Self::add_to_queue_string(&mut s, &self.now_playing.lock().await.as_ref().unwrap(), num, true);
        num +=1;

        for i in 0 .. after {
            Self::add_to_queue_string(&mut s, &s_queue[i], num, false);
            num +=1;
        }

        messager::send_normal(ctx, "Queue", s, false).await;
    }

    fn add_to_queue_string(s: &mut String, song: &Song, num: usize, selected: bool) {
        let selected_char = "➤";
        let selected_whitespace = "  ";
        let normal_whitespace = "     ";
        let number_style = format!("{num}) ");
        let song_str = song.to_string();

        if selected {
            s.push_str(&messager::bold(format!("{}{}{}{}\n", selected_char, selected_whitespace, number_style, song_str)));
        } else {
            s.push_str(&format!("{}{}{}\n", normal_whitespace, number_style, song_str));
        }

    }

    pub async fn is_queues_empty(&self) -> bool {
        self.song_queue.lock().await.is_empty() && self.repeat_queue.lock().await.is_empty()
    }

    pub async fn change_repeat_mode(&self, ctx: &Context<'_>, new_mode: &Repeat) {
        *self.repeat_mode.lock().await = *new_mode;
        messager::send_sucsess(ctx, format!("Repeat mode changed to {}", new_mode), false).await;
    }
}

// TODO: add repeat algorithm
#[derive(poise::ChoiceParameter, Copy, Clone)]
pub enum Repeat {
    Off,
    One,
    All,
}

