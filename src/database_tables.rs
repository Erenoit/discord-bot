//! Keeps the all strusts releted to database table(s).

/// Stores key-value pairs for database.
///
/// Generally used for storing link aliases in music command and `reqwest`
/// cookies.
pub struct KeyValue {
    /// alias/search term to get actual value
    pub key:   String,
    /// value corresponding to given key
    pub value: String,
}
