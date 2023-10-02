use base64;
use base64::Engine;
use std::io;

fn hamming_dist(a: &Vec<u8>, b: &Vec<u8>) -> u32 {
    let mut count = 0;

    if a.len() != b.len() {
        panic!("Arrays do not have same length");
    }

    for i in 0..a.len() {
        count += (a[i] ^ b[i]).count_ones();
    }

    count
}

fn main() {
    let mut input_base64 = String::new();
    loop {
        let mut line = String::new();
        let nchars = io::stdin().read_line(&mut line).unwrap();
        if nchars == 0 {
            break;
        }
        input_base64.push_str(line.trim());
    }

    let decoder = &base64::engine::general_purpose::STANDARD;
    let bytes = decoder.decode(&input_base64).unwrap();

    println!(
        "{}",
        hamming_dist(
            &"this is a test".as_bytes().to_vec(),
            &"wokka wokka!!!".as_bytes().to_vec()
        )
    )
}
