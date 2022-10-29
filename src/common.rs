use crate::player::Player;
use tokio::sync::Mutex;

pub struct Server {
    pub player: Mutex<Player>
}
