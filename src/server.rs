use serenity::model::id::GuildId;

use crate::player::Player;

#[non_exhaustive]
pub struct Server {
    pub player: Player,
}

impl Server {
    pub fn new(guild_id: GuildId) -> Self { Self { player: Player::new(guild_id) } }
}
