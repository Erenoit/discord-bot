pub mod others;
pub mod entertainment;
pub mod music;

use crate::common::Server;
use std::collections::HashMap;
use serenity::model::id::GuildId;

type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    pub servers: HashMap<GuildId, Server>,
}

