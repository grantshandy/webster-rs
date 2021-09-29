use chrono::prelude::*;

fn main() {
    println!("Benchmark without preload, the first test should be significantly longer.");
    let mut num = 1;

    while num < 6 {
        let start_duration = Utc::now();

        let _ = webster::dictionary("amazing").unwrap();

        let end_duration = Utc::now();

        let total_time = end_duration.signed_duration_since(start_duration).num_nanoseconds().unwrap();

        println!("test {}, {} nanoseconds", num, total_time);
        num += 1;
    }
}