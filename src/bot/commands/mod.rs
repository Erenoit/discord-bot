pub mod others;
pub mod entertainment;
pub mod music;

type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    pub command_counter: std::sync::Mutex<std::collections::HashMap<String, u64>>,
}

