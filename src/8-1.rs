use std::io::{self, Read};

fn main() {
    let mut img = String::new();
    io::stdin().read_to_string(&mut img).unwrap();

    let line = img
        .as_bytes()
        .chunks_exact(25 * 6)
        .min_by_key(|c| c.iter().filter(|&&b| b == b'0').count())
        .unwrap();
    let res =
        line.iter().filter(|&&b| b == b'1').count() * line.iter().filter(|&&b| b == b'2').count();
    println!("{}", res);
}
