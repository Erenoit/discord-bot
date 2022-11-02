use crate::{logger, server::Server};
use std::{collections::HashMap, env, sync::Arc};
use dotenv;
use serenity::model::id::GuildId;
use songbird::Songbird;

use tokio::sync::RwLock;

pub struct Config {
    token: String,
    prefix: String,
    servers: RwLock<HashMap<GuildId, Server>>,
    songbird: Arc<Songbird>,
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
        let songbird = Songbird::serenity();

        Self { token, prefix, servers, songbird }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
     pub fn prefix(&self) -> &String {
        &self.prefix
     }

     pub fn servers(&self) -> &RwLock<HashMap<GuildId, Server>> {
        &self.servers
     }

     pub fn songbird(&self) -> Arc<Songbird> {
         Arc::clone(&self.songbird)
     }
}
