use crate::get_config;
use songbird::{Event, EventContext, EventHandler};
use serenity::model::id::GuildId;

pub struct SongEnd {
    pub guild_id: GuildId
}

#[poise::async_trait]
impl EventHandler for SongEnd {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        match ctx {
            EventContext::Track(..) => {
                get_config()
                    .servers().read().await
                    .get(&self.guild_id).unwrap()
                    .player
                    .start_stream().await;
            }
            _ => unimplemented!("Unimplemented event occured")
        }

        None
    }
}

