use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

pub const CHAR_LOWER_A: u8 = 97;
pub const CHAR_LOWER_F: u8 = 102;
pub const CHAR_UPPER_A: u8 = 65;
pub const CHAR_UPPER_F: u8 = 70;
pub const CHAR_0: u8 = 48;
pub const CHAR_9: u8 = 57;

fn hex_character_to_u8(c: u8) -> Result<u8, String> {
    if CHAR_LOWER_A <= c && c <= CHAR_LOWER_F {
        return Ok(c - CHAR_LOWER_A + 10);
    }
    if CHAR_UPPER_A <= c && c <= CHAR_UPPER_F {
        return Ok(c - CHAR_UPPER_A + 10);
    }
    if CHAR_0 <= c && c <= CHAR_9 {
        return Ok(c - CHAR_0);
    }
    let msg = format!("Invalid char (code {c}) in input");
    Err(msg)
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
    if c == 64 {
        return '/';
    }
    panic!("Invalid 6-bit sextet {c}");
}

pub fn bytes_to_hex(bytes: &Vec<u8>) -> String {
    let mut output = String::new();
    for byte in bytes {
        let byte_str = format!("{:02x}", byte);
        output.push_str(&byte_str);
    }
    output
}

pub fn parse_hex_string(str: &str) -> Result<Vec<u8>, String> {
    let input_bytes = str.trim().as_bytes();
    let input_len = input_bytes.len();
    let byte_len = input_len / 2;
    if byte_len * 2 != input_len {
        let msg = format!("Input string {str} does not have a valid (even) number of characters");
        return Err(msg);
    }

    let mut result: Vec<u8> = Vec::with_capacity(byte_len);
    for i in 0..byte_len {
        let big = hex_character_to_u8(input_bytes[i * 2])?;
        let small = hex_character_to_u8(input_bytes[1 + (i * 2)])?;
        result.push(big << 4 | small);
    }
    Ok(result)
}

pub fn read_hex_line() -> Result<Vec<u8>, String> {
    let mut input = String::new();

    if let Ok(_) = io::stdin().read_line(&mut input) {
        parse_hex_string(&input)
    } else {
        let msg = String::from_str("Failed to read line as expected").unwrap();
        Err(msg)
    }
}

pub fn bytes_to_base64(bytes: &Vec<u8>) -> String {
    let mut result = String::new();

    for i in 0..bytes.len() / 3 {
        let b1 = u32::from(bytes[i * 3]);
        let b2 = u32::from(bytes[i * 3 + 1]);
        let b3 = u32::from(bytes[i * 3 + 2]);
        let word = b1 << 16 | b2 << 8 | b3;
        let c1 = u8::try_from(word >> 18).unwrap();
        let c2 = u8::try_from((word >> 12) & 0x3F).unwrap();
        let c3 = u8::try_from((word >> 6) & 0x3F).unwrap();
        let c4 = u8::try_from(word & 0x3F).unwrap();
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
        let c1 = u8::try_from(word >> 18).unwrap();
        let c2 = u8::try_from((word >> 12) & 0x3F).unwrap();
        result.push(sextet_to_base64_char(c1));
        result.push(sextet_to_base64_char(c2));
        result.push('=');
        result.push('=');
        result
    } else {
        let b1 = u32::from(bytes[regular_end]);
        let b2 = u32::from(bytes[regular_end + 1]);
        let word: u32 = b1 << 16 | b2 << 8;
        let c1 = u8::try_from(word >> 18).unwrap();
        let c2 = u8::try_from((word >> 12) & 0x3F).unwrap();
        let c3 = u8::try_from((word >> 6) | 0x3F).unwrap();
        result.push(sextet_to_base64_char(c1));
        result.push(sextet_to_base64_char(c2));
        result.push(sextet_to_base64_char(c3));
        result.push('=');
        result
    }
}

pub fn get_expected_frequencies() -> Result<[f64; 256], String> {
    let count_csv = Path::new("../frequency_counts.txt");
    let count_file = File::open(count_csv).unwrap();
    let count_file_lines = io::BufReader::new(count_file).lines();
    let mut total_freq: f64 = 0.0;
    let mut result: [f64; 256] = [0.0; 256];
    for line_or_error in count_file_lines {
        let untrimmed_line = line_or_error.unwrap();
        let line = untrimmed_line.trim();
        if let Some(index) = line.find(',') {
            let char_value: usize = line[..index].parse().unwrap();
            let freq_value: f64 = line[index + 1..].parse().unwrap();
            if result[usize::from(char_value)] != 0.0 {
                let msg = format!("Multiple frequency counts given for {char_value}");
                return Err(msg);
            }
            total_freq = total_freq + freq_value;
            result[usize::from(char_value)] = freq_value;
        }
    }
    for i in 0..256 {
        result[i] = result[i] / total_freq;
    }
    Ok(result)
}

pub fn test_string_frequencies(expected: &[f64; 256], str: &Vec<u8>) -> f64 {
    let mut actual: [f64; 256] = [0.0; 256];
    let len = str.len() as f64;

    for i in 0..str.len() {
        actual[usize::from(str[i])] = actual[usize::from(str[i])] + 1.0;
    }

    let mut error = 0.0;
    for i in 0..256 {
        let observed = actual[i];
        let expected_observations = expected[i] * len;
        let diff = observed - expected_observations;

        if observed > 0.0 && expected_observations == 0.0 {
            return std::f64::MAX;
        } else if expected_observations > 0.0 {
            error += diff * diff / expected_observations;
        }
    }
    error
}

fn xor_bytes_with_key(bytes: &[u8], key: &[u8], offset: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(bytes.len());

    for i in 0..bytes.len() {
        result.push(bytes[i] ^ key[(i + offset) % key.len()]);
    }

    result
}
