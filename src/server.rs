//! The module that contains the `Server` struct.

use reqwest::Client;
use serenity::model::id::GuildId;

#[cfg(feature = "music")]
use crate::player::Player;

// TODO: Create `Player` after first call for it.
/// The struct that contains all the information needed for one guild.
#[non_exhaustive]
pub struct Server {
    /// The player for the server.
    #[cfg(feature = "music")]
    pub player: Player,
}

impl Server {
    /// Creats new `Server` struct.
    #[cfg(feature = "music")]
    pub fn new(guild_id: GuildId, reqwest_client: Client) -> Self {
        Self {
            player: Player::new(guild_id, reqwest_client),
        }
    }

    #[cfg(not(feature = "music"))]
    pub const fn new(_guild_id: GuildId, _reqwest_client: Client) -> Self { Self {} }
}
