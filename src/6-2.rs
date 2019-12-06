use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let orbits = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| (l[4..=6].to_string(), l[0..=2].to_string()))
        .collect::<HashMap<_, _>>();

    let you = parents(&orbits, "YOU");
    let san = parents(&orbits, "SAN");
    let common = you.iter().position(|b| san.contains(b)).unwrap();
    let count = common + san.iter().position(|&b| b == you[common]).unwrap() + 2;

    println!("{}", count);
}

fn parents<'a>(orbits: &'a HashMap<String, String>, body: &str) -> Vec<&'a str> {
    let mut body = orbits.get(body).unwrap();
    let mut chain = Vec::new();

    while let Some(parent) = orbits.get(body) {
        chain.push(parent.as_str());
        body = parent;
    }

    chain
}
