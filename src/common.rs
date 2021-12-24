use std::io::BufRead;
use std::path::Path;
use std::{fs::File, io::BufReader};

pub fn read_lines<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("open file");
    BufReader::new(file).lines().map(|lines| lines.unwrap())
}

pub struct ReadChunks {
    buf: Vec<String>,
    lines: Box<dyn Iterator<Item = String>>,
}

impl ReadChunks {
    pub fn new<P: 'static>(filename: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            buf: vec![],
            lines: Box::new(read_lines(filename)),
        }
    }
}

impl Iterator for ReadChunks {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        for line in &mut self.lines {
            if line.is_empty() {
                break;
            }
            self.buf.push(line);
        }
        if self.buf.is_empty() {
            None
        } else {
            Some(std::mem::take(&mut self.buf))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProgramOutput {
    Halt,
    NeedInput,
    Output(i64),
}

#[derive(Clone)]
pub struct Program {
    pub codes: Vec<i64>,
    pub pc: usize,
    base: i64,
}

impl Program {
    pub fn from_file(name: &str) -> Self {
        Self::new(
            read_lines(name)
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>(),
        )
    }
    pub fn new(mut codes: Vec<i64>) -> Program {
        codes.extend([0; 1000]);
        Program {
            codes,
            pc: 0,
            base: 0,
        }
    }

    pub fn run_program(&mut self, mut input: Option<i64>) -> ProgramOutput {
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
                    // println!("OP_IN a={} in={:?} pc={}", a, input, pc);
                    if let Some(i) = input {
                        write_param(codes, a, mode % 10, self.base, i);
                        input = None;
                    } else {
                        self.pc = pc;
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
        self.pc = pc;
        ProgramOutput::Halt
    }
}

pub fn find_edge(points: impl IntoIterator<Item = (i32, i32)>) -> (i32, i32, i32, i32) {
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

pub fn shortest_path<T, F>(start: T, end: T, edges: F) -> Option<usize>
where
    T: std::hash::Hash + Eq + Clone + Copy + Ord,
    F: Fn(&T) -> Vec<(T, usize)>,
{
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashMap};
    let mut dist: HashMap<T, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(Reverse((0, start)));

    while let Some(Reverse((cost, pos))) = heap.pop() {
        if pos == end {
            return Some(cost);
        }
        if cost > dist[&pos] {
            continue;
        }
        for (edge, new_cost) in edges(&pos) {
            let new_cost = cost + new_cost;
            let d = dist.entry(edge).or_insert(usize::MAX);
            if new_cost < *d {
                heap.push(Reverse((new_cost, edge)));
                *d = new_cost;
            }
        }
    }
    None
}
