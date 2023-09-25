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

fn sextet_to_base64_char(c: u8) -> char {
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

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input_bytes = input.trim().as_bytes();

    let len = input_bytes.len();
    let mut data = Vec::with_capacity(len / 2);

    if len / 2 * 2 != len {
        println!("Invalid number of chars input");
    }

    println!("{} {} {}", input.len(), input_bytes.len(), data.len());
    for i in 0..len / 2 {
        let big = hex_byte_to_u8(input_bytes[2 * i]);
        let small = hex_byte_to_u8(input_bytes[2 * i + 1]);
        println!("{big},{small}");
        data.push(big << 4 | small);
    }

    for i in 0..data.len() / 3 {
        let x = u32::from(data[i * 3]);
        let y = u32::from(data[i * 3 + 1]);
        let z = u32::from(data[i * 3 + 2]);
        let bits = x << 16 | y << 8 | z;
        let a = u8::try_from(bits >> 18).expect("Invariant");
        let b = u8::try_from((bits >> 12) & 0x3F).expect("Invariant");
        let c = u8::try_from((bits >> 6) & 0x3F).expect("Invariant");
        let d = u8::try_from(bits & 0x3F).expect("Invariant");
        print!(
            "{}{}{}{}",
            sextet_to_base64_char(a),
            sextet_to_base64_char(b),
            sextet_to_base64_char(c),
            sextet_to_base64_char(d)
        );
    }
    println!("");
}


// SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
// SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t