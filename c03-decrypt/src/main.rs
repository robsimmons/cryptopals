use std::str;

// usage: cat test.txt | cargo run

fn main() {
    let expected = cryptobuddy::get_expected_frequencies().unwrap();

    let input = cryptobuddy::read_hex_line().unwrap();
    for ch in 0..128 {
        let mut copy = input.to_vec();
        for i in 0..copy.len() {
            copy[i] = copy[i] ^ ch;
        }
        let error = cryptobuddy::test_string_frequencies(&expected, &copy);
        println!("XOR with {ch} - error is {:.2}", error);
        if error < 1000.0 {
            let str = str::from_utf8(&copy).expect("Nope, didn't work");
            println!("decode - {str}")
        }
    }
}
