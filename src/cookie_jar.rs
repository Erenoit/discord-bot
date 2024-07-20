//! Cookie storage for [`reqwest`].
//!
//! Its database implementation is really hacky, but it is the only way I can
//! imagine that works without async. At least [`reqwest`] supports async in
//! [`CookieStore`] trait it will be this way.
//!
//! [`CookieStorage`]: reqwest::cookie::CookieStore

#[cfg(feature = "database")]
use std::{
    sync::{
        mpsc::{channel, Sender},
        Arc,
        Barrier,
        Mutex,
    },
    thread::JoinHandle,
};

use reqwest::{cookie::CookieStore, header::HeaderValue, Url};

#[cfg(feature = "database")]
use crate::database_tables::KeyValue;

// TODO: database feature path probably needs some optimizations. It copies a
// lot.

pub struct CookieJar {
    /// Thread that handles database operations.
    #[cfg(feature = "database")]
    #[allow(dead_code)]
    thread: JoinHandle<()>,
    /// Channel sender to send events to the thread.
    #[cfg(feature = "database")]
    sender: Sender<CookieEvent>,

    /// In memory storage for cookies when database is not used.
    #[cfg(not(feature = "database"))]
    storage: Vec<(String, String, String)>,
}

impl CookieJar {
    #[cfg(not(feature = "database"))]
    pub fn new() -> Self { CookieJar { storage: Vec::new() } }

    #[cfg(feature = "database")]
    pub fn new() -> Self {
        let (sender, receiver) = channel();

        // Spawning a regular thread gets it out of the tokio runtime; therefore, a new
        // runtime can be created in the thread for async operations.
        let thread = std::thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    loop {
                        let Ok(event) = receiver.recv() else {
                            break;
                        };

                        match event {
                            CookieEvent::Set(barier, cookie_headers, url) => {
                                Self::async_set_cookies(&mut cookie_headers.iter(), &url).await;
                                barier.wait();
                            },
                            CookieEvent::Get(barier, url, result) => {
                                let mut res_locked = result.lock().unwrap();
                                *res_locked = Self::async_cookies(&url).await;
                                barier.wait();
                            },
                        }
                    }

                    ()
                });
        });

        CookieJar { thread, sender }
    }

    #[cfg(feature = "database")]
    async fn async_set_cookies(cookie_headers: &mut dyn Iterator<Item = &HeaderValue>, url: &Url) {
        let database = get_config!()
            .database_pool()
            .expect("Always Some if database feature is enabled");

        let url = url.host_str().unwrap();

        for (key, value) in cookie_headers
            .flat_map(|header| {
                header
                    .to_str()
                    .expect("Cannot fail unless reqwest sent invalid cookie")
                    .split("; ")
            })
            .filter_map(|header| header.split_once('='))
        {
            sqlx::query!(
                "INSERT INTO cookies (site, key, value) VALUES (?, ?, ?)",
                url,
                key,
                value
            )
            .execute(database)
            .await
            .ok();
        }
    }

    #[cfg(feature = "database")]
    async fn async_cookies(url: &Url) -> Option<HeaderValue> {
        let database = get_config!()
            .database_pool()
            .expect("Always Some if database feature is enabled");

        let url = url.host_str().unwrap();

        sqlx::query_as!(
            KeyValue,
            "SELECT key, value FROM cookies WHERE site = ?",
            url
        )
        .fetch_all(database)
        .await
        .map(|cookies| {
            cookies.iter().fold(String::new(), |mut acc, cookie| {
                let adding_length = cookie.key.len() + cookie.value.len() + 3;

                acc.reserve(adding_length);
                if !acc.is_empty() {
                    acc.push_str("; ");
                }

                acc.push_str(&cookie.key);
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
    }
}

#[cfg(feature = "database")]
impl CookieStore for CookieJar {
    fn set_cookies(&self, cookie_headers: &mut dyn Iterator<Item = &HeaderValue>, url: &Url) {
        let barier = Arc::new(Barrier::new(2));

        let cookie_headers = cookie_headers.cloned().collect();

        self.sender
            .send(CookieEvent::Set(
                Arc::clone(&barier),
                cookie_headers,
                url.clone(),
            ))
            .expect("Always Some if database feature is enabled");

        barier.wait();
    }

    fn cookies(&self, url: &Url) -> Option<HeaderValue> {
        let result = Arc::new(Mutex::new(None));
        let barier = Arc::new(Barrier::new(2));

        self.sender
            .send(CookieEvent::Get(
                Arc::clone(&barier),
                url.clone(),
                Arc::clone(&result),
            ))
            .expect("Always Some if database feature is enabled");

        barier.wait();

        Arc::try_unwrap(result)
            .ok()
            .map(|r| r.into_inner().ok())
            .flatten()
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
                self.storage.push((
                    url.to_string(),
                    key.to_string(),
                    value.to_string(),
                ));
            });
    }

    fn cookies(&self, url: &Url) -> Option<HeaderValue> {
        let url = url.host_str().unwrap();

        Ok(
            self.storage.iter().filter(|(site, ..)| site == url).fold(
                String::new(),
                |mut acc, (_, key, value)| {
                    let adding_length = key.len() + value.len() + 3;

                    acc.reserve(adding_length);
                    if !acc.is_empty() {
                        acc.push_str("; ");
                    }

                    acc.push_str(key);
                    acc.push('=');
                    acc.push_str(value);

                    acc
                },
            ),
        )
    }
}

enum CookieEvent {
    Set(Arc<Barrier>, Vec<HeaderValue>, Url),
    Get(Arc<Barrier>, Url, Arc<Mutex<Option<HeaderValue>>>),
}
