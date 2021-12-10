use advent_of_code::common::read_lines;
fn main() {
    let codes = read_lines("./input5.txt")
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    println!(
        "{}",
        run_program(codes.clone(), [1]).into_iter().last().unwrap()
    );
    println!("{}", run_program(codes, [5]).into_iter().last().unwrap());
}

fn read_param(memory: &[i32], param: i32, mode: i32) -> i32 {
    // println!("param: {} mode:{}", param, mode);
    match mode {
        0 => memory[param as usize],
        1 => param,
        _ => panic!("Invalid mode"),
    }
}

fn run_program(mut codes: Vec<i32>, input: impl IntoIterator<Item = i32>) -> Vec<i32> {
    let mut input = input.into_iter();
    let mut output = vec![];
    let mut pc = 0;
    while codes[pc] != 99 {
        let opcode = codes[pc] % 100;
        let mode = codes[pc] / 100;
        // println!(
        //     "pc={} raw={} opcode={} mode={}",
        //     pc, codes[pc], opcode, mode
        // );
        match opcode {
            1 => {
                let a = read_param(&codes, codes[pc + 1], mode % 10);
                let b = read_param(&codes, codes[pc + 2], mode % 100 / 10);
                let c = codes[pc + 3];
                // println!("OP_ADD a={} b={} c={}", a, b, c);
                codes[c as usize] = a + b;
                pc += 4;
            }
            2 => {
                let a = read_param(&codes, codes[pc + 1], mode % 10);
                let b = read_param(&codes, codes[pc + 2], mode % 100 / 10);
                let c = codes[pc + 3];
                // println!("OP_MUL a={} b={} c={}", a, b, c);
                codes[c as usize] = a * b;
                pc += 4;
            }
            3 => {
                let a = codes[pc + 1];
                // println!("OP_IN a={}", a);
                codes[a as usize] = input.next().unwrap();
                pc += 2;
            }
            4 => {
                let a = read_param(&codes, codes[pc + 1], mode % 10);
                // println!("OP_OUT a={}", a);
                output.push(a);
                pc += 2;
            }
            5 => {
                let a = read_param(&codes, codes[pc + 1], mode % 10);
                let b = read_param(&codes, codes[pc + 2], mode % 100 / 10);
                // println!("OP_JNZ a={} b={}", a, b);
                if a != 0 {
                    pc = b as usize;
                } else {
                    pc += 3;
                }
            }
            6 => {
                let a = read_param(&codes, codes[pc + 1], mode % 10);
                let b = read_param(&codes, codes[pc + 2], mode % 100 / 10);
                // println!("OP_JZ a={} b={}", a, b);
                if a == 0 {
                    pc = b as usize;
                } else {
                    pc += 3;
                }
            }
            7 => {
                let a = read_param(&codes, codes[pc + 1], mode % 10);
                let b = read_param(&codes, codes[pc + 2], mode % 100 / 10);
                let c = codes[pc + 3];
                // println!("OP_LT a={} b={} c={}", a, b, c);
                codes[c as usize] = if a < b { 1 } else { 0 };
                pc += 4;
            }
            8 => {
                let a = read_param(&codes, codes[pc + 1], mode % 10);
                let b = read_param(&codes, codes[pc + 2], mode % 100 / 10);
                let c = codes[pc + 3];
                // println!("OP_EQ a={} b={} c={}", a, b, c);
                codes[c as usize] = if a == b { 1 } else { 0 };
                pc += 4;
            }
            99 => break,
            _ => panic!("Unknown opcode {}, pc={}", codes[pc], pc),
        }
    }
    output
}
