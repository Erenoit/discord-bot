//! Reddit API structs
//!
//! These structs are used to deserialize JSON responses from the Reddit rondom.

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RedditPost {
    pub post_link: String,
    pub title:     String,
    pub url:       String,
    pub ups:       i64,
}
