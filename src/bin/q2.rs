use advent_of_code::common::read_lines;
fn main() {
    let codes = read_lines("./input2.txt")
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    println!("{}", run_program(codes.clone(), 12, 2));
    for noun in 0..100 {
        for verb in 0..100 {
            if run_program(codes.clone(), noun, verb) == 19690720 {
                println!("{}", 100 * noun + verb);
                return;
            }
        }
    }
}
fn run_program(mut codes: Vec<u32>, noun: u32, verb: u32) -> u32 {
    codes[1] = noun;
    codes[2] = verb;
    let mut pc = 0;
    while codes[pc] != 99 {
        match codes[pc] {
            1 => {
                let a = codes[codes[pc + 1] as usize];
                let b = codes[codes[pc + 2] as usize];
                let c = codes[pc + 3] as usize;
                codes[c] = a + b;
            }
            2 => {
                let a = codes[codes[pc + 1] as usize];
                let b = codes[codes[pc + 2] as usize];
                let c = codes[pc + 3] as usize;
                codes[c] = a * b;
            }
            _ => panic!("Unknown opcode {}", codes[pc]),
        }
        pc += 4;
    }
    codes[0]
}
