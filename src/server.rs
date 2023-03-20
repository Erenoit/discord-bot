use serenity::model::id::GuildId;

#[cfg(feature = "music")]
use crate::player::Player;

#[non_exhaustive]
pub struct Server {
    #[cfg(feature = "music")]
    pub player: Player,
}

impl Server {
    pub fn new(guild_id: GuildId) -> Self {
        #[cfg(feature = "music")]
        {
            Self { player: Player::new(guild_id) }
        }

        #[cfg(not(feature = "music"))]
        Self {}
    }
}
