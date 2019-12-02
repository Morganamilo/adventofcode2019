use std::io::{self, Read};

fn main() {
    let mut ops = String::new();
    io::stdin().read_to_string(&mut ops).unwrap();
    let ops = ops
        .split(',')
        .map(|n| n.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let (n, v) = (0..100)
        .flat_map(move |n| (0..100).map(move |v| (n, v)))
        .find(|&(n, v)| run(&mut ops.clone(), n, v) == 19690720)
        .unwrap();
    println!("{}", 100 * n + v);
}

fn run(ops: &mut [usize], a: usize, b: usize) -> usize {
    ops[1..=2].copy_from_slice(&[a, b]);

    for ip in (0..).step_by(4) {
        match ops[ip] {
            1 => ops[ops[ip + 3]] = ops[ops[ip + 1]] + ops[ops[ip + 2]],
            2 => ops[ops[ip + 3]] = ops[ops[ip + 1]] * ops[ops[ip + 2]],
            99 => return ops[0],
            _ => panic!("invalid op code"),
        }
    }

    panic!("end of ops");
}
