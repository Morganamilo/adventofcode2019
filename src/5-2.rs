use std::io::{self, Read};

struct Program<'a> {
    ops: &'a mut [i32],
    ip: usize,
}

impl<'a> Program<'a> {
    fn arg(&self, arg: usize) -> i32 {
        if self.ops[self.ip] / 10_i32.pow(arg as u32 + 1) % 10 == 0 {
            self.ops[self.ops[arg + self.ip] as usize]
        } else {
            self.ops[arg + self.ip]
        }
    }

    fn set(&mut self, arg: usize, value: i32) {
        self.ops[self.ops[self.ip + arg] as usize] = value;
    }

    fn step(&mut self, value: i32) {
        self.ip += value as usize;
    }

    fn jmp(&mut self, ip: i32) {
        self.ip = ip as usize;
    }

    fn op(&self) -> i32 {
        self.ops[self.ip] % 100
    }
}

fn main() {
    let mut ops = String::new();
    io::stdin().read_to_string(&mut ops).unwrap();
    let mut ops = ops
        .split(',')
        .map(|n| n.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut p = Program {
        ops: ops.as_mut_slice(),
        ip: 0,
    };

    loop {
        match p.op() {
            1 => {
                p.set(3, p.arg(1) + p.arg(2));
                p.step(4);
            }
            2 => {
                p.set(3, p.arg(1) * p.arg(2));
                p.step(4);
            }
            3 => {
                p.set(1, 5);
                p.step(2);
            }
            4 => {
                println!("{}", p.arg(1));
                p.step(2);
            }
            5 => {
                if p.arg(1) != 0 {
                    p.jmp(p.arg(2));
                } else {
                    p.step(3);
                }
            }
            6 => {
                if p.arg(1) == 0 {
                    p.jmp(p.arg(2));
                } else {
                    p.step(3);
                }
            }
            7 => {
                p.set(3, (p.arg(1) < p.arg(2)) as i32);
                p.step(4);
            }
            8 => {
                p.set(3, (p.arg(1) == p.arg(2)) as i32);
                p.step(4);
            }
            99 => return,
            _ => panic!("invalid op code"),
        }
    }
}
