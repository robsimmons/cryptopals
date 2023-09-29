use std::fs;
use std::path::Path;

fn main() {
    let short_stories = Path::new("../../sherlock/stories");
    let mut freq = [0; 128];
    for entry in fs::read_dir(short_stories).expect("Dir did not exist") {
        let entry = entry.expect("Invalid DirEntry");
        let story_bytes = fs::read(entry.path()).expect("Path was not a readable file");
        eprintln!("{}", entry.path().display());
        for byte in story_bytes {
            if byte > 127 {
                eprintln!("Invalid byte {byte}");
            } else {
                freq[usize::from(byte)] += 1;
            }
        }
    }

    for i in 0..128 {
        println!("{i},{}", freq[i]);
    }

}
