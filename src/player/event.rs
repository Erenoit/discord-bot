//! Event handler for songbird events

use serenity::model::id::GuildId;
use songbird::{tracks::PlayMode, Event, EventContext, EventHandler};
use tracing::error;

/// Struct for cathing `songbird::events::track::TrackEvent::End`
/// When a track end it calls `Player::start_stream()` to start next song
pub struct SongEnd {
    /// Guild id for the server that the event occured
    pub guild_id: GuildId,
}

#[poise::async_trait]
impl EventHandler for SongEnd {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        match ctx {
            &EventContext::Track(track_list) => {
                if let Some((state, handle)) = track_list.first() {
                    if let PlayMode::Errored(ref err) = state.playing {
                        error!(
                            "[Guild {}] Track {:?} failed with error: {:?}",
                            self.guild_id,
                            handle.uuid(),
                            err
                        );
                    }
                }

                get_config!()
                    .servers()
                    .read()
                    .await
                    .get(&self.guild_id)?
                    .player
                    .start_stream()
                    .await;
            },
            _ => unimplemented!("Unimplemented event occured"),
        }

        None
    }
}
