mod song;

use crate::{bot::commands::Context, logger, messager, player::song::Song};
use serenity::model::id::{ChannelId, GuildId};

pub struct Player {
    guild_id: GuildId,
    connected_channel: Option<ChannelId>,
    now_playing:  Option<Song>,
    repeat_mode:  Repeat,
    song_queue:   Vec<Song>,
    repeat_queue: Vec<Song>,
}

impl Player {
    pub fn new(guild_id: GuildId) -> Self {
        Self {
            guild_id,
            connected_channel: None,
            now_playing:       None,
            repeat_mode:       Repeat::Off,
            song_queue:        Vec::with_capacity(100),
            repeat_queue:      Vec::with_capacity(100),
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
                self.connect_to_voice_channel(&ctx, &channel_id);
            } else {
                messager::send_error(ctx, "You are not in the voice channel", true);
                return;
            }
        }

        let s = Song::new(url, &ctx.author().name).await;
        self.song_queue.push(s);

        if self.now_playing.is_none() {
            self.start().await
        }
    }

    pub async fn start(&mut self) {
        if self.song_queue.is_empty() { self.stop(); return; }

        if let Some(call_mutex) = songbird::Songbird::get(self.guild_id) {
            let next_song = self.song_queue.pop().expect("Queue cannot be empty at this point");

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
        todo!()
    }

    pub async fn stop(&self) {
        if let Some(call_mutex) = get_call_mutex(ctx, guild_id).await {
            let mut call = call_mutex.lock().await;
            call.stop();
        }
        todo!()
    }
}

// TODO: add repeat algorithm
pub enum Repeat {
    Off,
    One,
    All,
}

