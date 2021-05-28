//! # webster-rs
//! A Rust library containing an offline version of webster's dictionary.
//!
//! Add to Cargo.toml
//! ```
//! webster = 0.2.1
//! ```
//!
//! Simple example:
//! ```rust
//! fn main() {
//!     let word = "silence";
//!
//!     let definition = webster::definition(word).unwrap();
//!
//!     println!("{} definition: {}", word, definition);
//! }
//! ```
//!
//! The definitions are not *great* but they'll do for simple projects if you need an open source local dictionary API.

#[macro_use]
extern crate lazy_static;
use serde_json::Value;

lazy_static! {
    pub static ref DICT: Value = serde_json::from_str(include_str!("dictionary.json")).unwrap_or(Value::from(""));
}

/// Translate a word
pub fn definition<T: AsRef<str>>(word: T) -> Option<&'static str> {
    match &DICT[word.as_ref().to_uppercase()] {
        Value::String(string) => return Some(string.as_str()),
        &_ => return None,
    }
}