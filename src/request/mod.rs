#[cfg(feature = "database")]
pub mod cookie_jar;
pub mod reddit_structs;
#[cfg(feature = "spotify")]
pub mod sp_structs;
pub mod yt_structs;

use std::sync::Arc;

use reqwest::Client;

#[cfg(feature = "database")]
use crate::request::cookie_jar::COOKIE_JAR;

/// User agent to use in requests
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:147.0) Gecko/20100101 Firefox/147.0";

/// Creates a new instance of [`reqwest::Client`] and configure.
pub fn create_reqwest_client() -> Client {
    let reqwest_client_builder = Client::builder()
        .user_agent(USER_AGENT)
        .use_rustls_tls()
        .https_only(true);

    #[cfg(feature = "database")]
    let reqwest_client_builder = reqwest_client_builder.cookie_provider(Arc::clone(&COOKIE_JAR));

    reqwest_client_builder
        .build()
        .expect("TLS backend cannot be initialized")
}
