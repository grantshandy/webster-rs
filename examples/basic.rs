fn main() {
    let word = "silence";

    let definition = webster::dictionary(word).unwrap();

    println!("{} definition: {}", word, definition);
}
