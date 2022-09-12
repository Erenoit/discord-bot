use crate::logger;

use dotenv;
use std::env;

pub(super) struct Config {
    token: String,
    prefix: String,
}

impl Config {
    pub fn generate() -> Self {
        logger::log("Registering Configs".to_string(), None);
        _ = dotenv::dotenv();

        logger::secondary_info("token".to_string());
        let token = env::var("TOKEN").expect("Couldn't find the token");

        logger::secondary_info("prefix".to_string());
        let prefix = env::var("PREFIX").expect("Couldn't find the prefix");

        Self { token, prefix }
    }

    pub fn token(&self) -> &String {
        return &self.token
    }
     pub fn prefix(&self) -> &String {
        return &self.prefix
     }
}
