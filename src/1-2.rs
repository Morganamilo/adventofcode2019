use std::io::{self, BufRead};

fn main() {
    let res = io::stdin()
        .lock()
        .lines()
        .filter_map(|l| l.unwrap().parse::<i32>().ok())
        .fold(0, |acc, n| acc + calculate(n / 3 - 2));
    println!("{}", res);
}

fn calculate(n: i32) -> i32 {
    if n <= 0 {
        0
    } else {
        n + calculate(n / 3 - 2)
    }
}
