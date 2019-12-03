use std::collections::HashMap;
use std::i32::MAX;
use std::io::{self, BufRead};

fn main() {
    let mut grid = HashMap::new();

    for (n, line) in io::stdin().lock().lines().map(Result::unwrap).enumerate() {
        let mut pos = (0, 0);
        let mut step = 0;

        for (dir, num) in line.split(',').map(|p| p.split_at(1)) {
            for _ in 0..num.parse::<i32>().unwrap() {
                match dir {
                    "R" => pos.0 += 1,
                    "D" => pos.1 += 1,
                    "L" => pos.0 -= 1,
                    "U" => pos.1 -= 1,
                    _ => panic!("not [RDLU]"),
                }

                let e = grid.entry(pos).or_insert([(false, MAX), (false, MAX)]);
                step += 1;
                e[n] = (true, e[n].1.min(step));
            }
        }
    }

    let min = grid
        .into_iter()
        .filter(|&(_, n)| n[0].0 && n[1].0)
        .map(|(_, s)| s[0].1 + s[1].1)
        .min();
    println!("{}", min.unwrap());
}
