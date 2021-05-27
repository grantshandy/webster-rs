use webster::dictionary;

fn main() {
    let word = dictionary("computer").unwrap();

    println!("computer definition: {}", word);
}