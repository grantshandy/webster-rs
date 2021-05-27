use webster::dictionary;

fn main() {
    let args = &std::env::args().collect::<Vec<_>>();
    let word = args[1].to_string();
    let definition = dictionary(word).unwrap();

    println!("{}", definition);
}