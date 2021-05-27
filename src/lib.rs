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

#[macro_use]
extern crate lazy_static;
use serde_json::Value;

lazy_static! {
    static ref DICT: Value = serde_json::from_str(include_str!("dictionary.json")).unwrap_or(Value::from(""));
}

/// Translate a word
pub fn new_dictionary<T: AsRef<str>>(word: T) -> Option<String> {
    match &DICT[word.as_ref().to_uppercase()] {
        Value::String(string) => return Some(string.to_string()),
        &_ => return None,
    }
}

/// Old Dictionary
pub fn old_dictionary<T: AsRef<str>>(word: T) -> Option<String> {
    let dict: Value = match serde_json::from_str(include_str!("dictionary.json")) {
        Ok(data) => data,
        Err(_) => return None,
    };

    match &dict[word.as_ref().to_uppercase()] {
        Value::String(string) => return Some(string.to_string()),
        &_ => return None,
    }
}

#[cfg(test)]
mod tests {
    // TAKES 5.23s
    #[test]
    fn old() {
        use crate::old_dictionary;

        old_dictionary("amazing").unwrap();
        old_dictionary("great").unwrap();
        old_dictionary("total").unwrap();
        old_dictionary("money").unwrap();
        old_dictionary("example").unwrap();
    }

    // TAKES 0.91s
    #[test]
    fn new() {
        use crate::new_dictionary;

        new_dictionary("amazing").unwrap();
        new_dictionary("great").unwrap();
        new_dictionary("total").unwrap();
        new_dictionary("money").unwrap();
        new_dictionary("example").unwrap();
    }
}