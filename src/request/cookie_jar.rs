//! Cookie storage for [`reqwest`].
//!
//! Its database implementation is really hacky, but it is the only way I can
//! imagine that works without async. At least [`reqwest`] supports async in
//! [`CookieStore`] trait it will be this way.
//!
//! [`CookieStorage`]: reqwest::cookie::CookieStore

use std::{
    env,
    path::Path,
    process,
    sync::{Arc, LazyLock},
    time::{SystemTime, UNIX_EPOCH},
};

use reqwest::{Url, cookie::CookieStore, header::HeaderValue};
use tokio::{
    fs::{self, File},
    io::{AsyncWriteExt, BufWriter},
};
use tracing::error;

use crate::database_tables::Cookie;

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
        let url = "https://youtube.com".parse::<Url>().expect("Always works");

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

    Arc::new(cookie_jar)
});

pub struct CookieJar {}

impl CookieJar {
    pub const fn new() -> Self { Self {} }

    #[expect(
        clippy::unused_self,
        reason = "to make sure that cookie jar is created beforehand"
    )]
    fn generate_netscape_file(&self) {
        let database = get_config!()
            .database_pool()
            .expect("Always Some if database feature is enabled");

        let file_generation = async move {
            fs::create_dir_all(unsafe {
                // # SAFETY: always have parent by initialization code
                Path::new(NETSCAPE_COOKIE_FILE_PATH.as_str())
                    .parent()
                    .unwrap_unchecked()
            })
            .await?;

            let f = File::create(NETSCAPE_COOKIE_FILE_PATH.as_str()).await?;
            let mut w = BufWriter::new(f);

            w.write_all(b"# Netscape HTTP Cookie File\n\n").await?;

            let cookies = sqlx::query_as!(
                Cookie,
                r#"SELECT host, host_only AS "host_only: bool", path, name, value,
                expires, secure AS "secure: bool" FROM cookiesv3"#,
            )
            .fetch_all(database)
            .await?;

            for cookie in cookies {
                w.write_all(cookie.as_netscape().as_bytes()).await?;
            }

            w.flush().await?;

            Ok(())
        };

        match tokio::runtime::Handle::try_current() {
            Ok(handle) => {
                handle.spawn(file_generation);
            },
            Err(_) => {
                std::thread::spawn(move || -> anyhow::Result<()> {
                    tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .expect("Failed to create fallback runtime for cookie file")
                        .block_on(file_generation)
                });
            },
        }
    }
}

impl CookieStore for CookieJar {
    fn set_cookies(&self, cookie_headers: &mut dyn Iterator<Item = &HeaderValue>, url: &Url) {
        let Some(host) = url.host_str() else {
            error!("URL without host: {}", url);
            return;
        };
        let host = host.to_ascii_lowercase();

        let cookies = cookie_headers
            .filter_map(|c| Cookie::from_header(c, &host, url.path()))
            .filter(|c| !c.secure || url.scheme() == "https")
            .collect::<Vec<_>>();

        if cookies.is_empty() {
            return;
        }

        let database_write = async move {
            let Some(database) = get_config!().database_pool() else {
                error!("Couldn't get database pool so no Cookie is saved");
                return;
            };

            for cookie in cookies {
                let res = sqlx::query!(
                    r#"
                    INSERT INTO cookiesv3 (host, host_only, path, name, value, expires, secure)
                    VALUES (?, ?, ?, ?, ?, ?, ?) ON CONFLICT(host, path, name) DO UPDATE SET
                        value = excluded.value,
                        expires = excluded.expires,
                        secure = excluded.secure,
                        host_only = excluded.host_only
                    "#,
                    cookie.host,
                    cookie.host_only,
                    cookie.path,
                    cookie.name,
                    cookie.value,
                    cookie.expires,
                    cookie.secure
                )
                .execute(database)
                .await;

                if let Err(e) = res {
                    error!(
                        "Failed to persist cookie {}-{}: {}",
                        cookie.host, cookie.name, e
                    );
                }
            }

            COOKIE_JAR.generate_netscape_file();
        };

        match tokio::runtime::Handle::try_current() {
            Ok(handle) => {
                handle.spawn(database_write);
            },
            Err(_) => {
                std::thread::spawn(move || {
                    tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .expect("Failed to create fallback runtime for cookies")
                        .block_on(database_write);
                });
            },
        }
    }

    fn cookies(&self, url: &Url) -> Option<HeaderValue> {
        let Some(host) = url.host_str() else {
            error!("URL without host: {}", url);
            return None;
        };
        let host = host.to_ascii_lowercase();
        let is_https = url.scheme() == "https";
        let url_path = url.path();

        let database_fetch = async move {
            let database = get_config!().database_pool()?;
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_or(0, |d| d.as_secs() as i64);

            let cookies = sqlx::query_as!(
                Cookie,
                r#"
                SELECT host, host_only AS "host_only: bool", path, name, value,
                expires, secure AS "secure: bool" FROM cookiesv3
                WHERE (expires IS NULL OR expires > ?)
                AND ((host_only = TRUE AND host = ?)
                    OR (host_only = FALSE AND (host = ? OR ? LIKE '%.' || host)))
                "#,
                now,
                host,
                host,
                host
            )
            .fetch_all(database)
            .await;

            let cookies = match cookies {
                Ok(c) => c,
                Err(e) => {
                    error!("Fai.led to load cookies: {}", e);
                    return None;
                },
            };

            let header = cookies
                .iter()
                .filter(|c| {
                    if c.path == url_path {
                        true
                    } else if let Some(rest) = url_path.strip_prefix(&c.path)
                        && (c.path.ends_with('/') || rest.starts_with('/'))
                    {
                        true
                    } else {
                        false
                    }
                })
                .filter(|c| !c.secure || is_https)
                .fold(String::new(), |mut acc, cookie| {
                    if !acc.is_empty() {
                        acc.push_str("; ");
                    }

                    acc.push_str(&cookie.name);
                    acc.push('=');
                    acc.push_str(&cookie.value);

                    acc
                });

            if header.is_empty() {
                return None;
            }

            HeaderValue::from_str(&header).ok()
        };

        match tokio::runtime::Handle::try_current() {
            Ok(handle) => tokio::task::block_in_place(|| handle.block_on(database_fetch)),
            Err(_) =>
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("Failed to create fallback runtime for cookies")
                    .block_on(database_fetch),
        }
    }
}
