use crate::{logger, server::Server};
use std::{collections::HashMap, env};
use dotenv;
use serenity::model::id::GuildId;

use tokio::sync::RwLock;

pub struct Config {
    token: String,
    prefix: String,
    servers: RwLock<HashMap<GuildId, Server>>,
    songbird: std::sync::Arc<songbird::Songbird>,
}

impl Config {
    pub fn generate() -> Self {
        logger::info("Registering Configs");
        _ = dotenv::dotenv();

        logger::secondary_info("token");
        let token = env::var("TOKEN").expect("Couldn't find the token");

        logger::secondary_info("prefix");
        let prefix = env::var("PREFIX").expect("Couldn't find the prefix");

        logger::secondary_info("empty servers hashmap");
        let servers = RwLock::new(HashMap::new());

        logger::secondary_info("songbird");
        let songbird = songbird::Songbird::serenity();

        Self { token, prefix, servers, songbird }
    }

    pub fn token(&self) -> &String {
        return &self.token
    }
     pub fn prefix(&self) -> &String {
        return &self.prefix
     }

     pub fn servers(&self) -> &RwLock<HashMap<GuildId, Server>> {
        return &self.servers;
     }

     pub fn songbird(&self) -> std::sync::Arc<songbird::Songbird> {
         return std::sync::Arc::clone(&self.songbird);
     }
}
