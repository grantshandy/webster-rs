# webster-rs
A Rust library containing an offline version of webster's dictionary.

```rust
use webster::dictionary;

fn main() {
  let word = dictionary("computer").unwrap();
  println!("computer definition: {}", word);
}
```

The definitions are not *great* but they'll do for simple projects if you need an open source local dictionary API.

This library uses the dictionary.json file from [adambom's dictionary](https://github.com/adambom/dictionary) adapted from Webster's Unabridged English Dictionary.

# Runtime Decompression
In an effort to reduce binary size (naive storage weighs `9mb`), the dictionary is stored in a compressed binary format in the executable (`4mb`)
and then decompressed upon runtime access. The runtime container provides `O(log n)` access complexity and access time (anecdotally) faster than a BTreeMap.

# License
The works in this repository are licensed under the MIT License, with the exception of the contents of dictionary.json, which are licensed under the terms of the Project Gutenberg License:

From Project Gutenberg:

> This eBook is for the use of anyone anywhere at no cost and with almost no restrictions whatsoever. You may copy it, give it away or re-use it under the terms of the Project Gutenberg License included with this eBook or online at www.gutenberg.net