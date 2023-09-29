use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{f64, u8, str};

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


fn get_frequency_counts() -> [f64; 256] {
    let count_csv = Path::new("../frequency_counts.txt");
    let count_file = File::open(count_csv).expect("Could not open file");
    let count_file_lines = io::BufReader::new(count_file).lines();
    let mut total_freq: f64 = 0.0;
    let mut result: [f64; 256] = [0.0; 256];
    for line_or_error in count_file_lines {
        let untrimmed_line = line_or_error.expect("File read error");
        let line = untrimmed_line.trim();
        if let Some(index) = line.find(',') {
            let char_value: usize = line[..index].parse().expect("Index not an int");
            let freq_value: f64 = line[index + 1..].parse().expect("Frequency not a number");
            if result[usize::from(char_value)] != 0.0 {
                panic!("Multiple frequency counts given for {char_value}");
            }
            total_freq = total_freq + freq_value;
            result[usize::from(char_value)] = freq_value;
        }
    }
    for i in 0..256 {
        result[i] = result[i] / total_freq;
    }
    result
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


fn frequency_counts_for_string(str: &Vec<u8>) -> [f64; 256] {
    let mut result: [f64; 256] = [0.0; 256];
    for i in 0..str.len() {
        result[usize::from(str[i])] = result[usize::from(str[i])] + 1.0;
    }
    for i in 0..result.len() {
        result[i] = result[i] / (str.len() as f64);
    }
    result
}

fn score(a: [f64; 256], b: [f64; 256]) -> f64 {
    let mut error: f64 = 0.0;
    for i in 0..256 {
        let diff = a[i] - b[i];
        error += diff * diff;
    }
    error
}

fn print_frequency_counts(counts: [f64; 256]) {
    for i in 0..128 {
        println!("{i}: {}", counts[i]);
    }
}

fn main() {
    let expected = get_frequency_counts();
    print_frequency_counts(expected);

    let input = read_hex_line();
    for ch in 0..128 {
        let mut copy = input.to_vec();
        for i in 0..copy.len() {
            copy[i] = copy[i] ^ ch;
        }
        let counts = frequency_counts_for_string(&copy);
        let error = score(expected, counts);
        println!("{ch} - {error}");
        if error < 0.05 {
            let str = str::from_utf8(&copy).expect("Nope, didn't work");
            println!("decode - {str}")
        }
    }
}
