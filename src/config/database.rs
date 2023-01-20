use std::{env, fs, path::PathBuf, process};

use rocksdb::{DBWithThreadMode, MultiThreaded, Options};
use taplo::dom::Node;

use crate::logger;

#[non_exhaustive]
pub(super) struct DatabaseConfig {
    connection: DBWithThreadMode<MultiThreaded>,
    options:    Options,
    path:       PathBuf,
}

impl DatabaseConfig {
    pub fn generate(config_file: &Node, default_path: PathBuf) -> Self {
        // TODO: make cmd argument priority over config file one
        let path = get_value!(config_file, PathBuf, "BOT_DATABASE_LOCATION", "database"=>"location", default_path);

        if !path.exists() {
            fs::create_dir_all(path.parent().expect(
                "it is safe to assume that this will always have a parent because we used join",
            ))
            .expect("directory creation should not fail in normal circumstances");
        }

        let mut options = Options::default();
        options.create_if_missing(true);
        options.create_missing_column_families(true);

        match DBWithThreadMode::open(&options, &path) {
            Ok(connection) => Self { connection, options, path },
            Err(why) => {
                logger::error("Couldn't open database.");
                logger::secondary_error(why);
                process::exit(1);
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
