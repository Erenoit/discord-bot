//! Keeps the all strusts releted to database table(s).

use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::header::HeaderValue;
use tracing::warn;

/// Stores key-value pairs for database.
///
/// Generally used for storing link aliases in music command
pub struct KeyValue {
    /// alias/search term to get actual value
    pub key:   String,
    /// value corresponding to given key
    pub value: String,
}

/// Stores cookie for database
///
/// Used for storing `reqwest` cookies
pub struct Cookie {
    pub host:      String,
    pub host_only: bool,
    pub path:      String,
    pub name:      String,
    pub value:     String,
    pub expires:   Option<i64>,
    pub secure:    bool,
}

impl Cookie {
    pub fn from_header(header: &HeaderValue, host: &str, path: &str) -> Option<Self> {
        let Ok(header_str) = header.to_str() else {
            warn!("Skipping cookie header with non-ASCII bytes");
            return None;
        };

        let mut cookie = Self::default();

        let mut segments = header_str
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty());

        let (name, value) = segments.next()?.split_once('=')?;
        name.trim().clone_into(&mut cookie.name);
        value.trim().clone_into(&mut cookie.value);

        let mut date_parsed = false;
        for segment in segments {
            if let Some((key, value)) = segment.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                if key.eq_ignore_ascii_case("path") {
                    if value.starts_with('/') {
                        value.clone_into(&mut cookie.path);
                    }
                } else if key.eq_ignore_ascii_case("domain") {
                    if !value.is_empty() {
                        value
                            .strip_prefix('.')
                            .unwrap_or(value)
                            .to_ascii_lowercase()
                            .clone_into(&mut cookie.host);
                    }
                } else if key.eq_ignore_ascii_case("max-age") {
                    let (neg, digits) = match value.strip_prefix('-') {
                        Some(rest) => (true, rest),
                        None => (false, value),
                    };

                    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
                        return None;
                    }

                    if let Ok(n) = digits.parse::<i64>() {
                        date_parsed = true;
                        cookie.expires = Some(
                            SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .map_or(0, |d| d.as_secs() as i64)
                                + if neg { -n } else { n },
                        );
                    }
                } else if !date_parsed && key.eq_ignore_ascii_case("expires") {
                    // TODO: expires field parsing
                    warn!("Expires field is found on cookie, skipptin the parsing of it.");
                }
                // NOTE: SameSite, Priority, Partitioned, etc. are ignored
            } else {
                if segment.eq_ignore_ascii_case("secure") {
                    cookie.secure = true;
                }
                // NOTE: HttpOnly is ignored
            }
        }

        // TODO:

        if cookie.host.is_empty() {
            cookie.host = host.trim().to_owned();
        }
        if cookie.path.is_empty() {
            cookie.path = path.trim().to_owned();
        }

        if cookie.host.is_empty() || cookie.name.is_empty() {
            None
        } else {
            Some(cookie)
        }
    }

    pub fn as_netscape(&self) -> String {
        format!(
            "{}{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            if !self.host_only { "." } else { "" },
            self.host,
            if !self.host_only { "TRUE" } else { "FALSE" },
            self.path,
            if self.secure { "TRUE" } else { "FALSE" },
            self.expires.unwrap_or(0),
            self.name,
            self.value
        )
    }
}

impl Default for Cookie {
    fn default() -> Self {
        Self {
            host:      String::new(),
            host_only: false,
            path:      String::new(),
            name:      String::new(),
            value:     String::new(),
            expires:   None,
            secure:    false,
        }
    }
}
