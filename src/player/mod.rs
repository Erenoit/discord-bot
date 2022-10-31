mod song;

use crate::{bot::commands::Context, logger, messager, player::song::Song};
use std::{collections::VecDeque, sync::Arc};
use serenity::model::id::{ChannelId, GuildId};
use songbird::{Call, Songbird};
use tokio::sync::Mutex;


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
                self.song_queue.append(&mut VecDeque::from(list));
            } else {
                messager::send_error(ctx, "Error happened while fetching data about playlist. Please try again later.", true).await;
                return;
            }
        } else if let Ok(s) = Song::new(url, &ctx.author().name).await {
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
}

// TODO: add repeat algorithm
pub enum Repeat {
    Off,
    One,
    All,
}

