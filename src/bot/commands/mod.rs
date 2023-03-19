#[macro_use]
mod macros;

pub mod entertainment;
pub mod music;
pub mod others;

type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data;
