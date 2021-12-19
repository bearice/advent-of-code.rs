use std::collections::HashMap;

use advent_of_code::common::read_lines;

fn main() {
    let codes = read_lines("./input11.txt")
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let program = Program::new(codes);
    let mut panels = HashMap::new();
    run_robot(&mut panels, program.clone());
    println!("{}", panels.len());

    let mut panels = HashMap::new();
    panels.insert((0, 0), 1);
    run_robot(&mut panels, program);
    draw_points(panels);
}

fn draw_points(points: HashMap<(i32, i32), i64>) {
    let (min_x, max_x, min_y, max_y) = find_edge(points.keys().cloned());
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if points.get(&(x, y)).unwrap_or(&0) == &1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn find_edge(points: impl IntoIterator<Item = (i32, i32)>) -> (i32, i32, i32, i32) {
    points
        .into_iter()
        .fold((i32::MAX, i32::MIN, i32::MAX, i32::MIN), |acc, x| {
            (
                std::cmp::min(acc.0, x.0),
                std::cmp::max(acc.1, x.0),
                std::cmp::min(acc.2, x.1),
                std::cmp::max(acc.3, x.1),
            )
        })
}

fn run_robot(panels: &mut HashMap<(i32, i32), i64>, mut program: Program) {
    let mut dir = 0;
    let mut pos = (0, 0);
    loop {
        let color = panels.entry(pos).or_insert(0);
        let new_color = program.run_program(Some(*color));
        let new_color = if let ProgramOutput::Output(new_color) = new_color {
            new_color
        } else {
            // println!("{:?}", new_color);
            break;
        };

        let new_dir = if let ProgramOutput::Output(new_dir) = program.run_program(None) {
            new_dir
        } else {
            panic!("unexpected output");
        };
        *color = new_color;
        dir += if new_dir == 0 { 3 } else { 1 };
        dir %= 4;
        match dir {
            0 => pos.1 -= 1,
            1 => pos.0 += 1,
            2 => pos.1 += 1,
            3 => pos.0 -= 1,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ProgramOutput {
    Halt,
    NeedInput,
    Output(i64),
}

#[derive(Clone)]
struct Program {
    codes: Vec<i64>,
    pc: usize,
    base: i64,
}

impl Program {
    fn new(mut codes: Vec<i64>) -> Program {
        codes.extend([0; 1000]);
        Program {
            codes,
            pc: 0,
            base: 0,
        }
    }

    fn run_program(&mut self, mut input: Option<i64>) -> ProgramOutput {
        fn read_param(memory: &[i64], param: i64, mode: i64, base: i64) -> i64 {
            // println!("param: {} mode:{}", param, mode);
            match mode {
                0 => memory[param as usize],
                1 => param,
                2 => memory[(base + param) as usize],
                _ => panic!("Invalid mode"),
            }
        }
        fn write_param(memory: &mut [i64], param: i64, mode: i64, base: i64, value: i64) {
            // println!("param: {} mode:{}", param, mode);
            match mode {
                0 => memory[param as usize] = value,
                2 => memory[(base + param) as usize] = value,
                _ => panic!("Invalid mode"),
            }
        }
        let codes = &mut self.codes;
        let mut pc = self.pc;
        while codes[pc] != 99 {
            let opcode = codes[pc] % 100;
            let mode = codes[pc] / 100;
            // println!(
            //     "pc={} raw={} opcode={} mode={} base={}",
            //     pc, codes[pc], opcode, mode, self.base
            // );
            match opcode {
                1 => {
                    let a = read_param(codes, codes[pc + 1], mode % 10, self.base);
                    let b = read_param(codes, codes[pc + 2], mode % 100 / 10, self.base);
                    let c = codes[pc + 3];
                    // println!("OP_ADD a={} b={} c={}", a, b, c);
                    write_param(codes, c, mode % 1000 / 100, self.base, a + b);
                    pc += 4;
                }
                2 => {
                    let a = read_param(codes, codes[pc + 1], mode % 10, self.base);
                    let b = read_param(codes, codes[pc + 2], mode % 100 / 10, self.base);
                    let c = codes[pc + 3];
                    // println!("OP_MUL a={} b={} c={}", a, b, c);
                    write_param(codes, c, mode % 1000 / 100, self.base, a * b);
                    pc += 4;
                }
                3 => {
                    let a = codes[pc + 1];
                    // println!("OP_IN a={}", a);
                    if let Some(i) = input {
                        write_param(codes, a, mode % 10, self.base, i);
                        input = None;
                    } else {
                        return ProgramOutput::NeedInput;
                    }
                    pc += 2;
                }
                4 => {
                    let a = read_param(codes, codes[pc + 1], mode % 10, self.base);
                    // println!("OP_OUT a={}", a);
                    pc += 2;
                    self.pc = pc;
                    return ProgramOutput::Output(a);
                }
                5 => {
                    let a = read_param(codes, codes[pc + 1], mode % 10, self.base);
                    let b = read_param(codes, codes[pc + 2], mode % 100 / 10, self.base);
                    // println!("OP_JNZ a={} b={}", a, b);
                    if a != 0 {
                        pc = b as usize;
                    } else {
                        pc += 3;
                    }
                }
                6 => {
                    let a = read_param(codes, codes[pc + 1], mode % 10, self.base);
                    let b = read_param(codes, codes[pc + 2], mode % 100 / 10, self.base);
                    // println!("OP_JZ a={} b={}", a, b);
                    if a == 0 {
                        pc = b as usize;
                    } else {
                        pc += 3;
                    }
                }
                7 => {
                    let a = read_param(codes, codes[pc + 1], mode % 10, self.base);
                    let b = read_param(codes, codes[pc + 2], mode % 100 / 10, self.base);
                    let c = codes[pc + 3];
                    // println!("OP_LT a={} b={} c={}", a, b, c);
                    write_param(
                        codes,
                        c,
                        mode % 1000 / 100,
                        self.base,
                        if a < b { 1 } else { 0 },
                    );

                    pc += 4;
                }
                8 => {
                    let a = read_param(codes, codes[pc + 1], mode % 10, self.base);
                    let b = read_param(codes, codes[pc + 2], mode % 100 / 10, self.base);
                    let c = codes[pc + 3];
                    // println!("OP_EQ a={} b={} c={}", a, b, c);
                    write_param(
                        codes,
                        c,
                        mode % 1000 / 100,
                        self.base,
                        if a == b { 1 } else { 0 },
                    );
                    pc += 4;
                }
                9 => {
                    let a = read_param(codes, codes[pc + 1], mode % 10, self.base);
                    // println!("OP_BASE a={}", a);
                    self.base += a;
                    pc += 2;
                }
                99 => break,
                _ => panic!("Unknown opcode {}, pc={}", codes[pc], pc),
            }
        }
        ProgramOutput::Halt
    }
}
