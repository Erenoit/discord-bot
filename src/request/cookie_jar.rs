//! Cookie storage for [`reqwest`].
//!
//! Its database implementation is really hacky, but it is the only way I can
//! imagine that works without async. At least [`reqwest`] supports async in
//! [`CookieStore`] trait it will be this way.
//!
//! [`CookieStorage`]: reqwest::cookie::CookieStore

#[cfg(not(feature = "database"))]
use std::sync::Mutex;

use reqwest::{cookie::CookieStore, header::HeaderValue, Url};

#[cfg(feature = "database")]
use crate::database_tables::KeyValue;

// TODO: database feature path probably needs some optimizations. It copies a
// lot.

pub struct CookieJar {
    /// In memory storage for cookies when database is not used.
    #[cfg(not(feature = "database"))]
    storage: Mutex<Vec<(String, String, String)>>,
}

impl CookieJar {
    #[cfg(not(feature = "database"))]
    pub fn new() -> Self {
        Self {
            storage: Mutex::new(Vec::new()),
        }
    }

    #[cfg(feature = "database")]
    pub fn new() -> Self { Self {} }
}

#[cfg(feature = "database")]
impl CookieStore for CookieJar {
    fn set_cookies(&self, cookie_headers: &mut dyn Iterator<Item = &HeaderValue>, url: &Url) {
        let cookie_headers = cookie_headers.cloned().collect::<Vec<_>>();
        let url = url.clone();

        // Spawning a regular thread gets it out of the tokio runtime; therefore, a new
        // runtime can be created in the thread for async operations.
        std::thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Cannot fail")
                .block_on(async {
                    let database = get_config!()
                        .database_pool()
                        .expect("Always Some if database feature is enabled");

                    let url = url.host_str().unwrap();

                    for (key, value) in cookie_headers
                        .iter()
                        .map(|header| {
                            // TODO: store expiration date
                            let h_str = header
                                .to_str()
                                .expect("Cannot fail unless reqwest sent invalid cookie");

                            h_str.split("; ").next().unwrap_or(h_str)
                        })
                        .filter_map(|header| header.split_once('='))
                    {
                        sqlx::query!(
                            "INSERT OR REPLACE INTO cookiesv2 (key, value) VALUES (? || ',' || ?, ?)",
                            url,
                            key,
                            value
                        )
                        .execute(database)
                        .await
                        .ok();
                    }
                })
        }).join().ok();
    }

    fn cookies(&self, url: &Url) -> Option<HeaderValue> {
        let url = url.clone();

        std::thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Cannot fail")
                .block_on(async {
                    let database = get_config!()
                        .database_pool()
                        .expect("Always Some if database feature is enabled");

                    let url = url.host_str().unwrap();

                    sqlx::query_as!(
                        KeyValue,
                        "SELECT key, value FROM cookiesv2 WHERE key LIKE ? || ',%'",
                        url
                    )
                    .fetch_all(database)
                    .await
                    .map(|cookies| {
                        cookies.iter().fold(String::new(), |mut acc, cookie| {
                            let actual_key = cookie.key.split_once(',').expect("Cannot fail").1;
                            let adding_length = actual_key.len() + cookie.value.len() + 3;

                            acc.reserve(adding_length);
                            if !acc.is_empty() {
                                acc.push_str("; ");
                            }

                            acc.push_str(actual_key);
                            acc.push('=');
                            acc.push_str(&cookie.value);

                            acc
                        })
                    })
                    .map(|cookie| {
                        HeaderValue::from_str(cookie.as_str())
                            .expect("Cannot fail unless reqwest sent invalid cookie")
                    })
                    .ok()
                })
        })
        .join()
        .ok()
        .flatten()
    }
}

#[cfg(not(feature = "database"))]
impl CookieStore for CookieJar {
    fn set_cookies(&self, cookie_headers: &mut dyn Iterator<Item = &HeaderValue>, url: &Url) {
        let url = url.host_str().unwrap();

        cookie_headers
            .flat_map(|header| {
                header
                    .to_str()
                    .expect("Cannot fail unless reqwest sent invalid cookie")
                    .split("; ")
            })
            .filter_map(|header| header.split_once('='))
            .for_each(|(key, value)| {
                self.storage.lock().unwrap().push((
                    url.to_string(),
                    key.to_string(),
                    value.to_string(),
                ));
            });
    }

    fn cookies(&self, url: &Url) -> Option<HeaderValue> {
        let url = url.host_str().unwrap();

        Some(
            HeaderValue::from_str(
                &self
                    .storage
                    .lock()
                    .unwrap()
                    .iter()
                    .filter(|(site, ..)| site == url)
                    .fold(String::new(), |mut acc, (_, key, value)| {
                        let adding_length = key.len() + value.len() + 3;

                        acc.reserve(adding_length);
                        if !acc.is_empty() {
                            acc.push_str("; ");
                        }

                        acc.push_str(key);
                        acc.push('=');
                        acc.push_str(value);

                        acc
                    }),
            )
            .unwrap(),
        )
    }
}
