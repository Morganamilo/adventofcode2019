use std::collections::HashSet;
use std::io::{self, Read};

#[derive(Clone)]
struct Program {
    ops: Vec<i32>,
    ip: usize,
}

impl Program {
    fn new(ops: Vec<i32>) -> Self {
        Self { ops, ip: 0 }
    }

    fn arg(&self, arg: usize) -> i32 {
        if self.ops[self.ip] / 10_i32.pow(arg as u32 + 1) % 10 == 0 {
            self.ops[self.ops[arg + self.ip] as usize]
        } else {
            self.ops[arg + self.ip]
        }
    }

    fn set(&mut self, arg: usize, value: i32) {
        let ops = self.ops.as_mut_slice();
        ops[ops[self.ip + arg] as usize] = value;
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

    fn run(&mut self, input: [i32; 2]) -> i32 {
        let mut input = input.iter().cloned();

        loop {
            match self.op() {
                1 => {
                    self.set(3, self.arg(1) + self.arg(2));
                    self.step(4);
                }
                2 => {
                    self.set(3, self.arg(1) * self.arg(2));
                    self.step(4);
                }
                3 => {
                    self.set(1, input.next().unwrap());
                    self.step(2);
                }
                4 => {
                    return self.arg(1);
                    self.step(2);
                }
                5 => {
                    if self.arg(1) != 0 {
                        self.jmp(self.arg(2));
                    } else {
                        self.step(3);
                    }
                }
                6 => {
                    if self.arg(1) == 0 {
                        self.jmp(self.arg(2));
                    } else {
                        self.step(3);
                    }
                }
                7 => {
                    self.set(3, (self.arg(1) < self.arg(2)) as i32);
                    self.step(4);
                }
                8 => {
                    self.set(3, (self.arg(1) == self.arg(2)) as i32);
                    self.step(4);
                }
                99 => panic!("should not end"),
                _ => panic!("invalid op code"),
            }
        }
    }
}
fn main() {
    let mut ops = String::new();
    io::stdin().read_to_string(&mut ops).unwrap();
    let mut ops = ops
        .split(',')
        .map(|n| n.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let p = Program::new(ops.clone());
    let mut max = 0;

    for a in (0..=4) {
        for b in (0..=4) {
            for c in (0..=4) {
                for d in (0..=4) {
                    for e in (0..=4) {
                        let nums = [a, b, c, d, e];
                        if nums.iter().collect::<HashSet<_>>().len() == 5 {
                            max = max.max((0..5).fold(0, |acc, n| p.clone().run([nums[n], acc])));
                        }
                    }
                }
            }
        }
    }

    println!("{}", max);
}
