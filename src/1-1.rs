use std::io::{self, BufRead};

fn main() {
    let res = io::stdin()
        .lock()
        .lines()
        .filter_map(|l| l.unwrap().parse::<i32>().ok())
        .fold(0, |acc, n| acc + n / 3 - 2);
    println!("{}", res);
}
