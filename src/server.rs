use serenity::model::id::GuildId;

#[cfg(feature = "music")]
use crate::player::Player;

// TODO: Create `Player` after first call for it.
/// The struct that contains all the information needed for one guild.
#[non_exhaustive]
pub struct Server {
    #[cfg(feature = "music")]
    pub player: Player,
}

impl Server {
    /// Creats new `Server` struct.
    #[cfg(feature = "music")]
    pub fn new(guild_id: GuildId) -> Self { Self { player: Player::new(guild_id) } }

    #[cfg(not(feature = "music"))]
    pub const fn new(_guild_id: GuildId) -> Self { Self {} }
}
