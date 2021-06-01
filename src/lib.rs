//! # webster-rs
//! A Rust library containing an offline version of webster's dictionary.
//!
//! Add to Cargo.toml
//! ```
//! webster = 0.3.0
//! ```
//!
//! Simple example:
//! ```rust
//! fn main() {
//!     let word = "silence";
//!
//!     let definition = webster::dictionary(word).unwrap();
//!
//!     println!("{} definition: {}", word, definition);
//! }
//! ```
//!
//! The definitions are not *great* but they'll do for simple projects if you need an open source local dictionary API.
//! 
//! # Runtime Decompression
//! In an effort to reduce binary size (naive storage weighs `9mb`), the dictionary is stored in a compressed binary format in the executable (`4mb`)
//! and then decompressed upon runtime access. The runtime container provides `O(log n)` access complexity and access time (anecdotally) faster than a BTreeMap.

use std::io::Read;
use lazy_static::{initialize, lazy_static};
use libflate::gzip::Decoder;

// this is the obscure 'formfeed' char, not found in the dictionary. 
// It's like a newline.
const DELIMITER: char = 0x0C as char;

// this is baked into the binary
static COMPRESSED_DICTIONARY: &[u8] = include_bytes!("dictionary.gz");

lazy_static! {
  static ref DICTIONARY: Vec<[String; 2]> = {
    // decompress binary representation
    let string = {
      let mut dec = Decoder::new(COMPRESSED_DICTIONARY).expect("Failed to initialize runtime dictionary decoder");
      let mut output = vec![];
      dec.read_to_end(&mut output).expect("Failed to decompress dictionary");
      String::from_utf8(output).expect("Failed to interpret decompressed data as string")
    };

    string
      .split(DELIMITER as char)
      .collect::<Vec<&str>>()
      .chunks_exact(2)
      .map(|s| [s[0].into(), s[1].into()])
      .collect::<Vec<[String; 2]>>()
  };
}

/// Decompress the binary-encoded dictionary ahead of time, so that all calls to `dictionary` are fast. 
/// The dictionary loading is managed for you, you don't need to call this to use `webster-rs`.
/// 
/// Without using this function, the first call will be much slower, as it needs to do the decompression
/// and parsing work.
pub fn preload() {
  initialize(&DICTIONARY);
}

/// Translate a word. O(log n).
pub fn dictionary<T: AsRef<str>>(word: T) -> Option<&'static str> {
  let key = word.as_ref().to_uppercase();
  DICTIONARY.binary_search_by(|[w, _]| w.cmp(&key))
    .ok()
    .and_then(|idx| Some(DICTIONARY[idx][1].as_str()))
}

mod test {
  #[test]
  fn test_define_rust() {
    assert_eq!(
      super::dictionary("rust"),
      Some("The reddish yellow coating formed on iron when exposed to moistair, consisting of ferric oxide or hydroxide; hence, by extension,any metallic film of corrosion.")
    )
  }
}