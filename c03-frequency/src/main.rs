use std::env;
use std::fs;

// usage: cargo run -- [files] > ../frequency_counts.txt
// example: cargo run -- ../../sherlock/**/*.txt > ../frequency_counts.txt

fn main() {
    let paths: Vec<String> = env::args().collect();

    let mut freq = [1; 256];
    for filename in &paths[1..] {
        eprintln!("Processing {filename}");
        let story_bytes = fs::read(&filename).expect("Path was not a readable file");
        for byte in story_bytes {
            freq[usize::from(byte)] += 1;
        }
    }

    for i in 0..256 {
        println!("{i},{}", freq[i]);
    }
}
