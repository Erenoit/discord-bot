pub mod others;
pub mod entertainment;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    pub command_counter: std::sync::Mutex<std::collections::HashMap<String, u64>>,
}

