use std::io;

const CHAR_LOWER_A: u8 = 97;
const CHAR_LOWER_F: u8 = 102;
const CHAR_UPPER_A: u8 = 65;
const CHAR_UPPER_F: u8 = 70;
const CHAR_0: u8 = 48;
const CHAR_9: u8 = 57;

fn hex_byte_to_u8(c: u8) -> u8 {
    if CHAR_LOWER_A <= c && c <= CHAR_LOWER_F {
        return c - CHAR_LOWER_A + 10;
    }
    if CHAR_UPPER_A <= c && c <= CHAR_UPPER_F {
        return c - CHAR_UPPER_A + 10;
    }
    if CHAR_0 <= c && c <= CHAR_9 {
        return c - CHAR_0;
    }
    panic!("Invalid char in input");
}

fn parse_hex(input: &str) -> Vec<u8> {
    let input_bytes = input.trim().as_bytes();
    let len = input_bytes.len();
    let mut data = Vec::with_capacity(len / 2);

    if len / 2 * 2 != len {
        println!("Invalid number of chars input {len}");
    }

    for i in 0..len / 2 {
        let big = hex_byte_to_u8(input_bytes[2 * i]);
        let small = hex_byte_to_u8(input_bytes[2 * i + 1]);
        data.push(big << 4 | small);
    }

    data
}

fn read_hex_line() -> Vec<u8> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    parse_hex(&input)
}

fn main() {
    let bytes_a = read_hex_line();
    let bytes_b = read_hex_line();

    if bytes_a.len() != bytes_b.len() {
        println!("Not equal lengths");
        return;
    }

    let mut output = Vec::with_capacity(bytes_a.len());
    for i in 0..bytes_a.len() {
        output.push(bytes_a[i] ^ bytes_b[i]);
    }

    for i in 0..bytes_a.len() {
        print!("{:X}", output[i]);
    }
}

// 746865206b696420646f6e277420706c6179
// 746865206B696420646F6E277420706C6179
