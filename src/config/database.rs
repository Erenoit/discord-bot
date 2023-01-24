use std::{env, fs, path::PathBuf};

use anyhow::{anyhow, Result};
use rocksdb::{DBWithThreadMode, MultiThreaded, Options};
use taplo::dom::Node;

#[non_exhaustive]
pub(super) struct DatabaseConfig {
    connection: DBWithThreadMode<MultiThreaded>,
    options:    Options,
    path:       PathBuf,
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

        let mut options = Options::default();
        options.create_if_missing(true);
        options.create_missing_column_families(true);

        match DBWithThreadMode::open(&options, &path) {
            Ok(connection) => Ok(Self { connection, options, path }),
            Err(why) => {
                log!(error, "Couldn't open database."; "{why}");
                Err(anyhow!("Couldn't open database."))
            },
        }
    }

    #[inline(always)]
    pub const fn connection(&self) -> &DBWithThreadMode<MultiThreaded> { &self.connection }

    // TODO: use this fuction somewhere makes sense
    #[allow(dead_code)]
    fn close_connection(self) {
        _ = DBWithThreadMode::<MultiThreaded>::destroy(&self.options, self.path);
    }
}
