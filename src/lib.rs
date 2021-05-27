//! # webster-rs
//! A Rust library containing an offline version of webster's dictionary.
//!
//! ```rust
//! use webster::dictionary;
//!
//! fn main() {
//!     let word = dictionary("computer").unwrap();
//!
//!     println!("computer definition: {}", word);
//! }
//! ```
//!
//! The definitions are not *great* but they'll do for simple projects if you need an open source local dictionary API.

use serde_json::Value;

/// Translate a word
pub fn dictionary<T: AsRef<str>>(word: T) -> Option<String> {
    let dict: Value = match serde_json::from_str(include_str!("dictionary.json")) {
        Ok(data) => data,
        Err(_) => return None,
    };

    match &dict[word.as_ref().to_uppercase()] {
        Value::String(string) => return Some(string.to_string()),
        &_ => return None,
    }
}