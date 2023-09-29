use cryptobuddy;

// usage: cat test.txt | cargo run

fn main() {
    let data = cryptobuddy::read_hex_line();
    println!("{}", cryptobuddy::bytes_to_base64(&data));
}