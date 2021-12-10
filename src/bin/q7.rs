use advent_of_code::common::read_lines;
use itertools::Itertools;

fn main() {
    let codes = read_lines("./input7.txt")
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let max = (0..=4)
        .into_iter()
        .permutations(5)
        .map(|x| amplify1(codes.clone(), &x))
        .max()
        .unwrap();
    println!("{}", max);
    let max = (5..=9)
        .into_iter()
        .permutations(5)
        .map(|x| amplify2(codes.clone(), &x))
        .max()
        .unwrap();
    println!("{}", max);
}

fn amplify1(codes: Vec<i32>, input: &[i32]) -> i32 {
    let mut i = 0;
    for x in input {
        i = run_program(&mut codes.clone(), 0, &mut (0, vec![*x, i]))
            .1
            .unwrap();
    }
    i
}

fn amplify2(codes: Vec<i32>, phase: &[i32]) -> i32 {
    let mut memory = (0..5).map(|_| codes.clone()).collect::<Vec<_>>();
    let mut pc = [0; 5];
    let mut i = 0;
    let mut signal = 0;
    let mut input = (0..5).map(|i| (0, vec![phase[i]])).collect::<Vec<_>>();
    loop {
        input[i].1.push(signal);
        let ret = run_program(&mut memory[i], pc[i], &mut input[i]);
        // println!("{:?}", ret);
        if ret.1.is_none() {
            // println!("i={} signal={}", i, signal);
            break;
        }
        pc[i] = ret.0;
        signal = ret.1.unwrap();
        i = (i + 1) % input.len();
    }
    signal
}

fn read_param(memory: &[i32], param: i32, mode: i32) -> i32 {
    // println!("param: {} mode:{}", param, mode);
    match mode {
        0 => memory[param as usize],
        1 => param,
        _ => panic!("Invalid mode"),
    }
}

fn run_program(
    codes: &mut Vec<i32>,
    mut pc: usize,
    input: &mut (usize, Vec<i32>),
) -> (usize, Option<i32>) {
    // let mut input = input.into_iter();
    // let mut pc = 0;
    while codes[pc] != 99 {
        let opcode = codes[pc] % 100;
        let mode = codes[pc] / 100;
        // println!(
        //     "pc={} raw={} opcode={} mode={}",
        //     pc, codes[pc], opcode, mode
        // );
        match opcode {
            1 => {
                let a = read_param(codes, codes[pc + 1], mode % 10);
                let b = read_param(codes, codes[pc + 2], mode % 100 / 10);
                let c = codes[pc + 3];
                // println!("OP_ADD a={} b={} c={}", a, b, c);
                codes[c as usize] = a + b;
                pc += 4;
            }
            2 => {
                let a = read_param(codes, codes[pc + 1], mode % 10);
                let b = read_param(codes, codes[pc + 2], mode % 100 / 10);
                let c = codes[pc + 3];
                // println!("OP_MUL a={} b={} c={}", a, b, c);
                codes[c as usize] = a * b;
                pc += 4;
            }
            3 => {
                let a = codes[pc + 1];
                // println!("OP_IN a={}", a);
                codes[a as usize] = input.1[input.0];
                input.0 += 1;
                pc += 2;
            }
            4 => {
                let a = read_param(codes, codes[pc + 1], mode % 10);
                // println!("OP_OUT a={}", a);
                pc += 2;
                return (pc, Some(a));
            }
            5 => {
                let a = read_param(codes, codes[pc + 1], mode % 10);
                let b = read_param(codes, codes[pc + 2], mode % 100 / 10);
                // println!("OP_JNZ a={} b={}", a, b);
                if a != 0 {
                    pc = b as usize;
                } else {
                    pc += 3;
                }
            }
            6 => {
                let a = read_param(codes, codes[pc + 1], mode % 10);
                let b = read_param(codes, codes[pc + 2], mode % 100 / 10);
                // println!("OP_JZ a={} b={}", a, b);
                if a == 0 {
                    pc = b as usize;
                } else {
                    pc += 3;
                }
            }
            7 => {
                let a = read_param(codes, codes[pc + 1], mode % 10);
                let b = read_param(codes, codes[pc + 2], mode % 100 / 10);
                let c = codes[pc + 3];
                // println!("OP_LT a={} b={} c={}", a, b, c);
                codes[c as usize] = if a < b { 1 } else { 0 };
                pc += 4;
            }
            8 => {
                let a = read_param(codes, codes[pc + 1], mode % 10);
                let b = read_param(codes, codes[pc + 2], mode % 100 / 10);
                let c = codes[pc + 3];
                // println!("OP_EQ a={} b={} c={}", a, b, c);
                codes[c as usize] = if a == b { 1 } else { 0 };
                pc += 4;
            }
            99 => break,
            _ => panic!("Unknown opcode {}, pc={}", codes[pc], pc),
        }
    }
    (pc, None)
}
