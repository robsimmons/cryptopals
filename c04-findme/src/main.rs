use cryptobuddy;
use std::str;

// usage: cat test.txt | cargo run

fn main() {
    let expected = cryptobuddy::get_expected_frequencies().unwrap();

    loop {
        let input = cryptobuddy::read_hex_line().unwrap();
        if input.len() == 0 {
            break;
        }

        let mut best = 0;
        let mut best_score = f64::MAX;
        for ch in 0..128 {
            let mut copy = input.to_vec();
            for i in 0..copy.len() {
                copy[i] = copy[i] ^ ch;
            }
            let score = cryptobuddy::test_string_frequencies(&expected, &copy);
            if score < best_score {
                best = ch;
                best_score = score;
            }
            if score < 1000.0 {
                let str = str::from_utf8(&copy).expect("Nope, didn't work");
                println!(
                    "decode - {} - {} - {str}",
                    cryptobuddy::bytes_to_hex(&input),
                    char::from(best)
                );
            }
        }

        println!("Best decode was with {best}, error was {:.2}", best_score);
    }
}
