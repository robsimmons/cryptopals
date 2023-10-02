use base64;
use base64::Engine;
use std::io;
use std::str;

fn main() {
    let expected = cryptobuddy::get_expected_frequencies().unwrap();

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

    let mut best_key_len = 0;
    let mut best_key_dist = 8.0;
    for key_len in 1..50 {
        let mut hamming_sum = 0;
        let mut hamming_comparison_count = 0;
        for i in 0..bytes.len() {
            let mut j = i;
            while j >= key_len {
                j -= key_len;
                hamming_sum += (bytes[i] ^ bytes[j]).count_ones();
                hamming_comparison_count += 1;
            }
        }
        let normalized_hamming_distance = (hamming_sum as f64) / (hamming_comparison_count as f64);
        println!(
            "For key length {key_len}, average hamming distance is {normalized_hamming_distance}",
        );
        if best_key_dist > normalized_hamming_distance {
            best_key_len = key_len;
            best_key_dist = normalized_hamming_distance;
        }
    }

    let mut transposed_bytes: Vec<Vec<u8>> = Vec::new();
    for _ in 0..best_key_len {
        transposed_bytes.push(Vec::new());
    }

    for i in 0..bytes.len() {
        transposed_bytes[i % best_key_len].push(bytes[i]);
    }

    let mut transposed_solution: Vec<Vec<u8>> = Vec::new();
    for i in 0..best_key_len {
        let vec = &transposed_bytes[i];

        let mut best = 0;
        let mut best_score = f64::MAX;
        let mut best_vec = Vec::new();
        let mut second_best_score = f64::MAX;
        for ch in 0..128 {
            let mut copy = vec.to_vec();
            for i in 0..copy.len() {
                copy[i] = copy[i] ^ ch;
            }
            let score = cryptobuddy::test_string_frequencies(&expected, &copy);
            if score < best_score {
                second_best_score = best_score;
                best_vec = Vec::from(copy);
                best = ch;
                best_score = score;
            }
        }
        //let str = str::from_utf8(best_vec).expect("Nope, didn't work");
        println!(
            "{} | score: {best_score} next best score: {second_best_score}",
            char::from(best)
        );
        transposed_solution.push(Vec::from(best_vec));
    }

    let mut solution = Vec::new();
    for i in 0..bytes.len() {
        solution.push(transposed_solution[i % best_key_len][i / best_key_len]);
    }

    let str = str::from_utf8(&solution).expect("Nope, didn't work");
    println!("{str}");
}
