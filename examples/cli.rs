fn main() {
    let word = std::env::args().nth(1).unwrap().to_string();

    let definition = webster::dictionary(word).unwrap();
  
    println!("{}", definition);
}