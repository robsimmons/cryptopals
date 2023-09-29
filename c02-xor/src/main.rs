use cryptobuddy;

// usage: cat test.txt | cargo run

fn main() {
    let bytes_a = cryptobuddy::read_hex_line().expect("Could not read first hex line");
    let bytes_b = cryptobuddy::read_hex_line().expect("Could not read second hex line");

    if bytes_a.len() != bytes_b.len() {
        println!("Lines not equal lengths");
        return;
    }

    let mut output = Vec::with_capacity(bytes_a.len());
    for i in 0..bytes_a.len() {
        output.push(bytes_a[i] ^ bytes_b[i]);
    }

    print!("{}", cryptobuddy::bytes_to_hex(&output));
}
