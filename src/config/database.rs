use std::{env, fs, path::PathBuf};

use anyhow::{anyhow, Result};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use taplo::dom::Node;

#[non_exhaustive]
pub(super) struct DatabaseConfig {
    pool: SqlitePool,
    url:  String,
}

impl DatabaseConfig {
    pub fn generate(config_file: &Node, default_path: PathBuf) -> Result<Self> {
        // TODO: make cmd argument priority over config file one
        let path = get_value!(config_file, PathBuf, "BOT_DATABASE_LOCATION", "database"=>"location", default_path)?;

        if !path.exists() {
            fs::create_dir_all(path.parent().ok_or_else(|| {
                anyhow!(
                    "it is safe to assume that this will always have a parent because we used join",
                )
            })?)?;
        }

        let url = format!(
            "sqlite://{}/discord_bot.db",
            path.to_string_lossy()
        );
        let pool: SqlitePool = SqlitePoolOptions::new().connect_lazy(&url)?;

        Ok(Self { pool, url })
    }

    pub const fn pool(&self) -> &SqlitePool { &self.pool }
}
