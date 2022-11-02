pub mod others;
pub mod entertainment;
pub mod music;

use serenity::model::guild::Guild;

type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data;


