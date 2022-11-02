mod song;

use crate::{bot::commands::Context, logger, messager, player::song::Song};
use std::{collections::{HashMap, VecDeque}, sync::Arc};
use serenity::model::id::{ChannelId, GuildId};
use songbird::{Call, Songbird};
use tokio::sync::Mutex;
use anyhow::anyhow;


#[inline(always)]
async fn get_songbird_manager(ctx: &Context<'_>) -> Arc<Songbird> {
    songbird::get(ctx.discord())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone()
}

#[inline(always)]
async fn get_call_mutex(ctx: &Context<'_>, guild_id: &GuildId) -> Option<Arc<Mutex<Call>>> {
    get_songbird_manager(ctx).await.get(*guild_id)
}

#[inline(always)]
fn context_to_voice_channel_id(ctx: &Context<'_>) -> Option<ChannelId> {
    ctx.guild().expect("Guild should be Some")
                .voice_states.get(&ctx.author().id)
                .and_then(|voice_state| voice_state.channel_id)
}

pub struct Player {
    guild_id: GuildId,
    connected_channel: Option<ChannelId>,
    now_playing:  Option<Song>,
    repeat_mode:  Repeat,
    song_queue:   VecDeque<Song>,
    repeat_queue: VecDeque<Song>,
}

impl Player {
    pub fn new(guild_id: GuildId) -> Self {
        Self {
            guild_id,
            connected_channel: None,
            now_playing:       None,
            repeat_mode:       Repeat::Off,
            song_queue:        VecDeque::with_capacity(100),
            repeat_queue:      VecDeque::with_capacity(100),
        }
    }

    pub async fn connect_to_voice_channel(&mut self, ctx: &Context<'_>, channel_id: &ChannelId) {
        let manager = songbird::get(ctx.discord()).await
            .expect("Songbird Voice client placed in at initialisation.").clone();

        let (call_mutex, result) = manager.join(self.guild_id, *channel_id).await;

        if let Err(why) = result {
            logger::error("Couldn't join the voice channel.");
            logger::secondary_error(why);
        } else {
            self.connected_channel = Some(*channel_id);

            let mut call = call_mutex.lock().await;
            if let Err(why) = call.deafen(true).await {
                logger::error("Couldn't deafen the bot.");
                logger::secondary_error(why);
            }
        }
    }

    pub async fn leave_voice_channel(&mut self, ctx: &Context<'_>) {
        if self.connected_channel == None {
            messager::send_error(ctx, "Not in a voice channel", true).await;
            return;
        }

        if let Some(call_mutex) = get_call_mutex(ctx, &self.guild_id).await {
            let mut call = call_mutex.lock().await;

            call.leave().await.expect("There shold be no error while leaving the call");
            self.connected_channel = None;
        }
    }

    pub async fn play(&mut self, ctx: &Context<'_>, url: String) {
        if self.connected_channel.is_none() {
            if let Some(channel_id) = context_to_voice_channel_id(ctx) {
                self.connect_to_voice_channel(&ctx, &channel_id).await;
            } else {
                messager::send_error(ctx, "You are not in the voice channel", true).await;
                return;
            }
        }

        if url.contains("list=") || url.contains("/playlist/") {
            if let Ok(list) = Song::from_playlist(url, &ctx.author().name).await {
                // CHECK: if "Vec -> VecDeque" reallocates the memmory
                messager::send_sucsess(ctx, format!("{} songs added to the list", list.len()), true).await;
                self.song_queue.append(&mut VecDeque::from(list));
            } else {
                messager::send_error(ctx, "Error happened while fetching data about playlist. Please try again later.", true).await;
                return;
            }
        } else if let Ok(s) = Song::new(url, &ctx.author().name).await {
            messager::send_sucsess(ctx, format!("{} is added to the list", s.title()), true).await;
            self.song_queue.push_back(s);
        } else {
            messager::send_error(ctx, "Error happened while fetching data about song. Please try again later.", true).await;
            return;
        }

        if self.now_playing.is_none() {
            self.start(ctx).await
        }
    }

    pub async fn start(&mut self, ctx: &Context<'_>) {
        if self.song_queue.is_empty() { self.stop(ctx).await; return; }

        if let Some(call_mutex) = get_call_mutex(ctx, &self.guild_id).await {
            let next_song = self.song_queue.pop_front().expect("Queue cannot be empty at this point");

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

            self.now_playing = Some(next_song);
        } else {
            unreachable!("Not in a voice channel to play in")
        }
    }

    pub async fn stop(&mut self, ctx: &Context<'_>) {
        if let Some(call_mutex) = get_call_mutex(ctx, &self.guild_id).await {
            let mut call = call_mutex.lock().await;
            call.stop();
            self.now_playing = None;
        }
    }

    pub async fn clear_the_queues(&mut self) {
        self.song_queue   = VecDeque::with_capacity(100);
        self.repeat_queue = VecDeque::with_capacity(100);
    }

    pub async fn shuffle_song_queue(&mut self) {
        for i in 0 ..= self.song_queue.len() - 2 {
          let j = (rand::random::<f32>() * (i as f32 - 1.0)) as usize;
          self.song_queue.swap(i, j);
        }
    }

    pub async fn print_queue(&self, ctx: &Context<'_>) {
        if self.now_playing.is_none() {
            messager::send_error(ctx, "Nothings playing :unamused:", true).await;
            return;
        }

        let mut s = String::with_capacity(1024);
        let s_len = self.song_queue.len();
        let r_len = self.repeat_queue.len();
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
            Self::add_to_queue_string(&mut s, &self.repeat_queue[i], num, false);
            num +=1;
        }

        Self::add_to_queue_string(&mut s, &self.now_playing.as_ref().unwrap(), num, true);
        num +=1;

        for i in 0 .. after {
            Self::add_to_queue_string(&mut s, &self.song_queue[i], num, false);
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

    pub fn is_queues_empty(&self) -> bool {
        self.song_queue.is_empty() && self.repeat_queue.is_empty()
    }

    pub async fn change_repeat_mode(&mut self, ctx: &Context<'_>, new_mode: &Repeat) {
        self.repeat_mode = *new_mode;
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

