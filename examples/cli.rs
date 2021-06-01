fn main() {
    let word = &std::env::args().collect::<Vec<_>>()[1].to_string();

    let definition = webster::definition(word).unwrap();
  
    println!("{}", definition);
}