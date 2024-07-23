pub mod cookie_jar;
pub mod reddit_structs;
#[cfg(feature = "spotify")]
pub mod sp_structs;
pub mod yt_structs;

use std::sync::Arc;

use reqwest::Client;
#[cfg(feature = "music")]
use reqwest::{cookie::CookieStore, Url};

use crate::request::cookie_jar::CookieJar;

/// User agent to use in requests
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:128.0) Gecko/20100101 Firefox/128.0";

/// Creates a new instance of [`reqwest::Client`] and configure.
pub fn create_reqwest_client() -> Client {
    let reqwest_client_builder = Client::builder()
        .user_agent(USER_AGENT)
        .use_rustls_tls()
        .https_only(true);

    let cookie_jar = CookieJar::new();

    #[cfg(feature = "music")]
    {
        let url = "https://www.youtube.com"
            .parse::<Url>()
            .expect("Always works");

        let yt_cookies = get_config!().youtube_cookies();
        let saved_cookies = cookie_jar.cookies(&url);
        if !yt_cookies.is_empty() && (saved_cookies.is_none() || saved_cookies.unwrap() == "") {
            let c = yt_cookies
                .split("; ")
                .map(|cookie| reqwest::header::HeaderValue::from_str(cookie).expect("Cannot fail"))
                .collect::<Vec<_>>();
            cookie_jar.set_cookies(&mut c.iter(), &url);
        }
    }

    let reqwest_client_builder = reqwest_client_builder.cookie_provider(Arc::new(cookie_jar));

    reqwest_client_builder
        .build()
        .expect("TLS backend cannot be initialized")
}
