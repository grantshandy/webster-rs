use webster::old_dictionary;

fn main() {
    let args = &std::env::args().collect::<Vec<_>>();
    let word = args[1].to_string();
    let definition = old_dictionary(word).unwrap();

    println!("{}", definition);
}