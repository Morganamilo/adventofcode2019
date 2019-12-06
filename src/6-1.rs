use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let mut count = 0;
    let orbits = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| (l[4..=6].to_string(), l[0..=2].to_string()))
        .collect::<HashMap<_,_>>();

    for mut body in orbits.keys() {
        while let Some(parent) = orbits.get(body) {
            count += 1;
            body = parent;
        }
    }

    println!("{}", count);
}
