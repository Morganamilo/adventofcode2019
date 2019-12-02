use std::io::{self, Read};

fn main() {
    let mut ops = String::new();
    io::stdin().read_to_string(&mut ops).unwrap();
    let mut ops = ops
        .split(',')
        .map(|n| n.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    ops[1] = 12;
    ops[2] = 2;

    for ip in (0..).step_by(4) {
        let o = ops[ip + 3];

        match ops[ip] {
            1 => ops[o] = ops[ops[ip + 1]] + ops[ops[ip + 2]],
            2 => ops[o] = ops[ops[ip + 1]] * ops[ops[ip + 2]],
            99 => break,
            _ => panic!("invalid op code"),
        }
    }

    println!("{}", ops[0]);
}
