use std::io::{self, Read};

fn main() {
    let mut r = String::new();
    io::stdin().read_to_string(&mut r).unwrap();
    let mut r = r.split('-').map(|n| n.trim().parse::<i32>().unwrap());
    let matches = (r.next().unwrap()..=r.next().unwrap())
        .map(|n| n.to_string().bytes().collect::<Vec<_>>())
        .filter(|n| {
            (0..n.len()).any(|i| {
                n[i + 1..].iter().filter(|&&c| c == n[i]).count()
                    + n[..i].iter().filter(|&&c| c == n[i]).count()
                    == 1
            }) && n.windows(2).all(|n| n[0] <= n[1])
        });
    println!("{}", matches.count());
}
