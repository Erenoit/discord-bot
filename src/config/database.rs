//! Database Configuration

use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
#[cfg(feature = "config_file")]
use taplo::dom::Node;

#[cfg(not(feature = "config_file"))]
use crate::config::Node;

/// Database configuration.
#[non_exhaustive]
pub(super) struct DatabaseConfig {
    /// Database pool.
    pool: SqlitePool,
    /// Database URL/location.
    url:  String,
}

impl DatabaseConfig {
    /// Generate a new `DatabaseConfig` from the config file.
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
            // create db file if not exists
            // <https://github.com/launchbadge/sqlx/issues/1114#issuecomment-827815038>
            "sqlite://{}/discord_bot.db?mode=rwc",
            path.to_string_lossy()
        );
        let pool: SqlitePool = SqlitePoolOptions::new().connect_lazy(&url)?;

        Ok(Self { pool, url })
    }

    /// Run database migrations.
    pub async fn run_migrations(&self) -> Result<()> {
        sqlx::migrate!().run(&self.pool).await?;
        Ok(())
    }

    /// Returns the database pool.
    pub const fn pool(&self) -> &SqlitePool { &self.pool }

    /// Returns the database URL.
    pub const fn url(&self) -> &String { &self.url }
}
