# webster-rs
A Rust library containing an offline version of webster's dictionary.

Add to Cargo.toml
```
webster = "0.3.0"
```

Simple example:
```rust
fn main() {
    let word = "silence";

    let definition = webster::dictionary(word).unwrap();

    println!("{} definition: {}", word, definition);
}
```

The definitions are not *great* but they'll do for simple projects if you need an open source local dictionary API.

This library uses the dictionary.json file from [adambom's dictionary](https://github.com/adambom/dictionary) adapted from Webster's Unabridged English Dictionary.

I'm currently working on my own program to parse the original public domain dictionary, but that's a [work in progress](https://github.com/grantshandy/webster-dictionary-json).

## Runtime Decompression
In an effort to reduce binary size (naive storage weighs `9mb`), the dictionary is stored in a compressed gzip binary format in the executable (`4mb`)
and then decompressed upon runtime access. The runtime container provides `O(log n)` access complexity and access time (anecdotally) faster than a BTreeMap.

## License
The works in this repository are licensed under the MIT License, with the exception of the contents of dictionary.json, which are licensed under the terms of the Project Gutenberg License:

From Project Gutenberg:

> This eBook is for the use of anyone anywhere at no cost and with almost no restrictions whatsoever. You may copy it, give it away or re-use it under the terms of the Project Gutenberg License included with this eBook or online at www.gutenberg.net
