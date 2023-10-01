use cryptobuddy;
use std::io;

// usage: cat test.txt | cargo run

fn main() {
    let mut input = String::new();

    let mut offset = 0;
    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        if input.len() == 0 {
            println!("");
            return;
        }
        let bytes = input.as_bytes();
        let key = "ICE".as_bytes();
        let encoded = xor_bytes_with_key(bytes, key, offset);
        print!("{}", cryptobuddy::bytes_to_hex(&encoded));
        offset += input.len();
    }
}
