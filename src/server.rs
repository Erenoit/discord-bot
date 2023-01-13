use crate::player::Player;
use serenity::model::id::GuildId;

#[non_exhaustive]
pub struct Server {
    pub player: Player
}

impl Server {
    pub fn new(guild_id: GuildId) -> Self {
        Self {
            player: Player::new(guild_id)
        }
    }
}
