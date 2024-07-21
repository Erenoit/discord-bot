//! Reddit API structs
//!
//! These structs are used to deserialize JSON responses from the Reddit rondom.

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RedditPost {
    pub data: RedditPostData,
}

#[derive(Deserialize, Serialize)]
pub struct RedditPostData {
    pub children: Vec<RedditPostDataHolder>,
}

#[derive(Deserialize, Serialize)]
pub struct RedditPostDataHolder {
    pub data: RedditPostData2,
}

#[derive(Deserialize, Serialize)]
pub struct RedditPostData2 {
    pub title:                  Option<String>,
    pub permalink:              String,
    pub url_overridden_by_dest: Option<String>,
    pub ups:                    i64,
    pub num_comments:           Option<i64>,
}
