use libflate::gzip::Encoder;
use std::{collections::HashMap, fs, io::Write};

// this is the obscure 'formfeed' char, not found in the dictionary.
// It's like a newline.
const DELIMITER: char = 0x0C as char;

fn main() {
    // Tell Cargo that if the dictionary file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/dictionary.json");

    // load the dictionary in, sort alphabetically by word
    let dict = {
        let dict_str = fs::read_to_string("./src/dictionary.json")
            .expect("Failed to read `dictionary.json` file");

        let mut dict = serde_json::from_str::<HashMap<String, String>>(&dict_str)
            .expect("Failed to parse `dictionary.json` file")
            .iter()
            .map(|(a, b)| (a.clone(), b.clone()))
            .collect::<Vec<(String, String)>>();

        dict.sort_by(|(word_a, _), (word_b, _)| word_a.cmp(&word_b));
        dict
    };

    // create an intermediate uncompressed format, slightly more efficient than JSON
    let mut ir = String::new();
    for (word, definition) in dict.iter() {
        let word = word.to_uppercase();
        let cleaned_word = word.trim();
        let cleaned_def = definition.trim();

        ir += &format!("{}{}{}{}", cleaned_word, DELIMITER, cleaned_def, DELIMITER);
    }

    // for some reason running `gzip -9 --keep ./src/dictionary.ir` (on the ir) gives better compression by like, 600kb,
    // - if you are a good person you will figure out how to correct the compressor config / algorithm used here
    let compressed = {
        let mut encoder = Encoder::new(Vec::new()).expect("Failed to create compressor");

        encoder
            .write_all(ir.as_bytes())
            .expect("Failed to compress dictionary");

        encoder
            .finish()
            .into_result()
            .expect("Failed to compress dictionary")
    };

    let target_dir = std::env::var("OUT_DIR").unwrap();

    fs::write(format!("{}/{}", target_dir, "dictionary.gz"), compressed).expect(&format!(
        "Failed to write compressed dictionary to `{}/dictionary.gz`",
        target_dir
    ));
}
