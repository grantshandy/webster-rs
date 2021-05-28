fn main() {
    let word = "silence";

    let definition = webster::definition(word).unwrap();

    println!("{} definition: {}", word, definition);
}