use webster::old_dictionary;

fn main() {
    let word = old_dictionary("computer").unwrap();

    println!("computer definition: {}", word);
}