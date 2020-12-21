use std::{collections::HashSet, str::FromStr};

use advent_of_code::common::read_lines;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl Instruction {
    fn exec(&self, acc: &mut isize, pc: isize) -> isize {
        match self {
            Self::Nop(_) => pc + 1,
            Self::Acc(x) => {
                *acc += x;
                pc + 1
            }
            Self::Jmp(x) => pc + x,
        }
    }
    fn arg(&self) -> isize {
        *match self {
            Self::Nop(x) => x,
            Self::Acc(x) => x,
            Self::Jmp(x) => x,
        }
    }
}
impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = s.split_ascii_whitespace();
        let cmd = v.next().unwrap();
        let arg = v.next().unwrap().parse().unwrap();
        match cmd {
            "nop" => Ok(Self::Nop(arg)),
            "jmp" => Ok(Self::Jmp(arg)),
            "acc" => Ok(Self::Acc(arg)),
            _ => Err(s.to_owned()),
        }
    }
}

fn check(instructions: &Vec<Instruction>) -> (isize, isize) {
    // println!("{:?}", instructions);
    let mut pc: isize = 0;
    let mut acc: isize = 0;
    let mut visited = HashSet::new();
    while !visited.contains(&pc) && (pc as usize) < instructions.len() {
        visited.insert(pc);
        let i = &instructions[pc as usize];
        // println!("{} {:?}", pc, i);
        pc = i.exec(&mut acc, pc);
        // println!("{:?}", visited);
    }
    // println!("{}", acc);
    (pc, acc)
}

fn main() {
    let mut instructions: Vec<_> = read_lines("./input8.txt")
        .unwrap()
        .map(Result::unwrap)
        .map(|s| Instruction::from_str(&s))
        .map(Result::unwrap)
        .collect();

    for i in 0..instructions.len() {
        let old = instructions[i];
        let new = match &old {
            Instruction::Jmp(x) => Instruction::Nop(*x),
            Instruction::Nop(x) => Instruction::Jmp(*x),
            _ => continue,
        };
        instructions[i] = new;
        let ret = check(&instructions);
        // println!("{:?}", ret);
        if ret.0 as usize == instructions.len() {
            println!("i={} inst={:?} acc={}", i, old, ret.1);
            break;
        }
        instructions[i] = old;
    }
}
