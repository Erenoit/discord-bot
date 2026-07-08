//! Cookie storage for [`reqwest`].
//!
//! Its database implementation is really hacky, but it is the only way I can
//! imagine that works without async. At least [`reqwest`] supports async in
//! [`CookieStore`] trait it will be this way.
//!
//! [`CookieStorage`]: reqwest::cookie::CookieStore

use std::{
    env,
    process,
    sync::{Arc, LazyLock},
};
#[cfg(not(feature = "database"))]
use std::{
    fs::File,
    io::{BufWriter, Write as _},
    sync::Mutex,
};

use reqwest::{cookie::CookieStore, header::HeaderValue, Url};
#[cfg(feature = "database")]
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
};

#[cfg(feature = "database")]
use crate::database_tables::KeyValue;

// TODO: database feature path probably needs some optimizations. It copies a
// lot.

pub static NETSCAPE_COOKIE_FILE_PATH: LazyLock<String> = LazyLock::new(|| {
    let mut path = env::temp_dir();
    path.push(process::id().to_string());
    path.push("cookies.txt");
    path.into_os_string().to_string_lossy().to_string()
});

pub static COOKIE_JAR: LazyLock<Arc<CookieJar>> = LazyLock::new(|| {
    let cookie_jar = CookieJar::new();

    #[cfg(feature = "music")]
    {
        let url = "https://www.youtube.com"
            .parse::<Url>()
            .expect("Always works");

        let yt_cookies = get_config!().youtube_cookies();
        let saved_cookies = cookie_jar.cookies(&url);
        if !yt_cookies.is_empty()
            && (saved_cookies.is_none() || saved_cookies.expect("Already checked").is_empty())
        {
            let c = yt_cookies
                .split("; ")
                .map(|cookie| reqwest::header::HeaderValue::from_str(cookie).expect("Cannot fail"))
                .collect::<Vec<_>>();
            cookie_jar.set_cookies(&mut c.iter(), &url);
        }
    }

    cookie_jar.generate_netscape_file();

    Arc::new(cookie_jar)
});

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
    pub const fn new() -> Self { Self {} }
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

                    let Some(url) = url.host_str() else {
                        return;
                    };

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
                });
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

                    let url = url.host_str()?;

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

impl CookieJar {
    #[cfg(feature = "database")]
    fn generate_netscape_file(&self) {
        std::thread::spawn(move || -> anyhow::Result<()> {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Cannot fail")
                .block_on(async {
                    let f = File::create(NETSCAPE_COOKIE_FILE_PATH.as_str()).await?;
                    let mut w = BufWriter::new(f);

                    w.write_all(b"# Netscape HTTP Cookie File\n\n");

                    let database = get_config!()
                        .database_pool()
                        .expect("Always Some if database feature is enabled");

                    let cookies = sqlx::query_as!(KeyValue, "SELECT key, value FROM cookiesv2",)
                        .fetch_all(database)
                        .await?;

                    cookies.iter().for_each(|cookie| {
                        let (url, key) = cookie.key.split_once(',').expect("Cannot fail");

                        w.write_all(
                            format!(
                                "{}\tTRUE\t/\tTRUE\t0\t{}\t{}\n",
                                url, key, cookie.value,
                            )
                            .as_bytes(),
                        );
                    });

                    w.flush();

                    Ok(())
                })
        })
        .join()
        .ok();
    }

    #[cfg(not(feature = "database"))]
    fn generate_netscape_file(&self) {
        let Ok(f) = File::create(NETSCAPE_COOKIE_FILE_PATH.as_str()) else {
            return;
        };
        let mut w = BufWriter::new(f);

        w.write_all(b"# Netscape HTTP Cookie File\n\n");

        &self
            .storage
            .lock()
            .unwrap()
            .iter()
            .for_each(|(url, key, value)| {
                w.write_all(format!("{}\tTRUE\t/\tTRUE\t0\t{}\t{}\n", url, key, value,).as_bytes());
            });

        w.flush();
    }
}
