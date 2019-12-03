use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let mut grid = HashMap::new();

    for (n, line) in io::stdin().lock().lines().map(Result::unwrap).enumerate() {
        let (mut x, mut y) = (0, 0);

        for (dir, num) in line.split(',').map(|p| p.split_at(1)) {
            for _ in 0..num.parse::<i32>().unwrap() {
                match dir {
                    "R" => x += 1,
                    "D" => y += 1,
                    "L" => x -= 1,
                    "U" => y -= 1,
                    _ => panic!("not [RDLU]"),
                }

                grid.entry((x, y)).or_insert([false, false])[n] = true;
            }
        }
    }

    let min = grid
        .into_iter()
        .filter(|&(_, n)| n[0] && n[1])
        .map(|((x, y), _)| i32::abs(x) + i32::abs(y))
        .min();
    println!("{}", min.unwrap());
}
