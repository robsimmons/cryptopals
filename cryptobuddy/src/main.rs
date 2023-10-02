use base64;
use base64::Engine;
use std::env;
use std::fs;

fn main() {
    let args = env::args().collect();
    let file = fs::read(&filename[1]).unwrap();
    let decoder = &base64::engine::general_purpose::STANDARD_NO_PAD; 
    let bytes = decoder.decode(&file).unwrap();
    println!("{}", cryptobuddy::bytes_to_hex(&bytes));
}
