use std::io;

pub const CHAR_LOWER_A: u8 = 97;
pub const CHAR_LOWER_F: u8 = 102;
pub const CHAR_UPPER_A: u8 = 65;
pub const CHAR_UPPER_F: u8 = 70;
pub const CHAR_0: u8 = 48;
pub const CHAR_9: u8 = 57;

pub fn hex_character_to_u8(c: u8) -> u8 {
    if CHAR_LOWER_A <= c && c <= CHAR_LOWER_F {
        return c - CHAR_LOWER_A + 10;
    }
    if CHAR_UPPER_A <= c && c <= CHAR_UPPER_F {
        return c - CHAR_UPPER_A + 10;
    }
    if CHAR_0 <= c && c <= CHAR_9 {
        return c - CHAR_0;
    }
    panic!("Invalid char (code {c}) in input");
}

pub fn sextet_to_base64_char(c: u8) -> char {
    if c < 26 {
        return (c + CHAR_UPPER_A) as char;
    }
    if c < 52 {
        return (c - 26 + CHAR_LOWER_A) as char;
    }
    if c < 62 {
        return (c - 52 + CHAR_0) as char;
    }
    if c == 63 {
        return '+';
    }
    return '/';
}

pub fn parse_hex_string(str: &str) -> Vec<u8> {
    let input_bytes = str.trim().as_bytes();
    let input_len = input_bytes.len();
    let byte_len = input_len / 2;
    if byte_len * 2 != input_len {
        panic!("Input string {str} does not have a valid (even) number of characters");
    }

    let mut result: Vec<u8> = Vec::with_capacity(byte_len);
    for i in 0..byte_len {
        let big = hex_character_to_u8(input_bytes[i * 2]);
        let small = hex_character_to_u8(input_bytes[1 + (i * 2)]);
        result.push(big << 4 | small);
    }
    result
}

pub fn read_hex_line() -> Vec<u8> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line as expected");

    parse_hex_string(&input)
}

pub fn bytes_to_base64(bytes: &Vec<u8>) -> String {
    let mut result = String::new();

    for i in 0..bytes.len() / 3 {
        let b1 = u32::from(bytes[i * 3]);
        let b2 = u32::from(bytes[i * 3 + 1]);
        let b3 = u32::from(bytes[i * 3 + 2]);
        let word = b1 << 16 | b2 << 8 | b3;
        let c1 = u8::try_from(word >> 18).expect("Invariant");
        let c2 = u8::try_from((word >> 12) & 0x3F).expect("Invariant");
        let c3 = u8::try_from((word >> 6) & 0x3F).expect("Invariant");
        let c4 = u8::try_from(word & 0x3F).expect("Invariant");
        result.push(sextet_to_base64_char(c1));
        result.push(sextet_to_base64_char(c2));
        result.push(sextet_to_base64_char(c3));
        result.push(sextet_to_base64_char(c4));
    }

    let regular_end = bytes.len() / 3 * 3;

    if regular_end == bytes.len() {
        result
    } else if regular_end + 1 == bytes.len() {
        let b1 = u32::from(bytes[regular_end]);
        let word: u32 = b1 << 16;
        let c1 = u8::try_from(word >> 18).expect("Invariant");
        let c2 = u8::try_from((word >> 12) & 0x3F).expect("Invariant");
        result.push(sextet_to_base64_char(c1));
        result.push(sextet_to_base64_char(c2));
        result.push('=');
        result.push('=');
        result
    } else {
        let b1 = u32::from(bytes[regular_end]);
        let b2 = u32::from(bytes[regular_end + 1]);
        let word: u32 = b1 << 16 | b2 << 8;
        let c1 = u8::try_from(word >> 18).expect("Invariant");
        let c2 = u8::try_from((word >> 12) & 0x3F).expect("Invariant");
        let c3 = u8::try_from((word >> 6) | 0x3F).expect("Invariant");
        result.push(sextet_to_base64_char(c1));
        result.push(sextet_to_base64_char(c2));
        result.push(sextet_to_base64_char(c3));
        result.push('=');
        result
    }
}
