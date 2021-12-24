use std::collections::HashSet;

use advent_of_code::common::read_lines;

type State = [i64; 5];
trait AsRegId {
    fn as_reg_id(&self) -> usize;
    fn as_reg_or_val(&self) -> RegOrVal;
}
impl AsRegId for &str {
    fn as_reg_id(&self) -> usize {
        match *self {
            "x" => 1,
            "y" => 2,
            "z" => 3,
            "w" => 4,
            _ => panic!("Invalid register name"),
        }
    }
    fn as_reg_or_val(&self) -> RegOrVal {
        match self.parse::<i64>() {
            Ok(v) => RegOrVal::Val(v),
            Err(_) => RegOrVal::Reg(self.as_reg_id()),
        }
    }
}
#[derive(Debug)]
enum RegOrVal {
    Reg(usize),
    Val(i64),
}
impl RegOrVal {
    fn get_val(&self, state: &State) -> i64 {
        match self {
            RegOrVal::Reg(reg) => state[*reg],
            RegOrVal::Val(val) => *val,
        }
    }
}
/*
inp a - Read an input value and write it to variable a.
add a b - Add the value of a to the value of b, then store the result in variable a.
mul a b - Multiply the value of a by the value of b, then store the result in variable a.
div a b - Divide the value of a by the value of b, truncate the result to an integer, then store the result in variable a. (Here, "truncate" means to round the value toward zero.)
mod a b - Divide the value of a by the value of b, then store the remainder in variable a. (This is also called the modulo operation.)
eql a b - If the value of a and b are equal, then store the value 1 in variable a. Otherwise, store the value 0 in variable a.
 */
#[derive(Debug)]
enum OpCode {
    Inp(usize),
    Add(usize, RegOrVal),
    Mul(usize, RegOrVal),
    Div(usize, RegOrVal),
    Mod(usize, RegOrVal),
    Eql(usize, RegOrVal),
}

impl OpCode {
    fn from_str(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        let op = parts.next().unwrap();
        let a = parts.next().unwrap().as_reg_id();
        if op == "inp" {
            return OpCode::Inp(a);
        }
        let b = parts.next().unwrap().as_reg_or_val();
        match op {
            "add" => OpCode::Add(a, b),
            "mul" => OpCode::Mul(a, b),
            "div" => OpCode::Div(a, b),
            "mod" => OpCode::Mod(a, b),
            "eql" => OpCode::Eql(a, b),
            _ => panic!("Invalid opcode"),
        }
    }
    fn exec(&self, state: &mut State) {
        // println!("op:{:?} st:{:?}", self, state);
        match self {
            OpCode::Inp(reg) => {
                state[*reg] = state[0];
            }
            OpCode::Add(reg, val) => {
                state[*reg] += val.get_val(state);
            }
            OpCode::Mul(reg, val) => {
                state[*reg] *= val.get_val(state);
            }
            OpCode::Div(reg, val) => {
                state[*reg] /= val.get_val(state);
            }
            OpCode::Mod(reg, val) => {
                state[*reg] %= val.get_val(state);
            }
            OpCode::Eql(reg, val) => {
                state[*reg] = if val.get_val(state) == state[*reg] {
                    1
                } else {
                    0
                };
            }
        }
    }
}

struct Block {
    id: usize,
    code: Vec<OpCode>,
}

impl Block {
    fn new(id: usize) -> Self {
        Block {
            id,
            code: Vec::new(),
        }
    }
    fn add_opcode(&mut self, line: &str) -> bool {
        if line.starts_with("inp") && !self.code.is_empty() {
            false
        } else {
            let opcode = OpCode::from_str(line);
            self.code.push(opcode);
            true
        }
    }
    fn run(&self, input: i64, prev_z: i64) -> i64 {
        let mut state = [input, 0, 0, prev_z, 0];
        for opcode in &self.code {
            opcode.exec(&mut state);
        }
        let z = state[3];
        if self.id == 0 {
            println!("block: {} input: {} => z: {}", self.id, input, z);
        }
        z
    }
}

fn search_one(
    cache: &mut HashSet<(usize, i64)>,
    blocks: &[Block],
    input: &[[i64; 9]],
    z: i64,
) -> Option<Vec<i64>> {
    if cache.contains(&(blocks[0].id, z)) {
        return None;
    }
    for i in input[0] {
        let new_z = blocks[0].run(i, z);
        if blocks.len() == 1 {
            if new_z == 0 {
                return Some(vec![i]);
            }
        } else if let Some(mut found) = search_one(cache, &blocks[1..], &input[1..], new_z) {
            found.push(i);
            return Some(found);
        }
    }
    cache.insert((blocks[0].id, z));
    None
}
fn search_first(cache: &mut HashSet<(usize, i64)>, blocks: &[Block], input: &[i64; 9]) -> Vec<i64> {
    let inputs = (0..blocks.len()).map(|_| *input).collect::<Vec<_>>();
    let ret = search_one(cache, blocks, &inputs, 0);
    ret.unwrap()
}

fn main() {
    let lines = read_lines("input24.txt");
    let mut blocks = Vec::new();
    let mut current_block = Block::new(0);
    for line in lines {
        if !current_block.add_opcode(&line) {
            blocks.push(current_block);
            current_block = Block::new(blocks.len());
            current_block.add_opcode(&line);
        }
    }
    blocks.push(current_block);

    let mut cache = HashSet::new();
    let max: String = search_first(&mut cache, &blocks, &[9, 8, 7, 6, 5, 4, 3, 2, 1])
        .iter()
        .map(ToString::to_string)
        .rev()
        .collect();
    println!("max: {}", max);
    let min: String = search_first(&mut cache, &blocks, &[1, 2, 3, 4, 5, 6, 7, 8, 9])
        .iter()
        .map(ToString::to_string)
        .rev()
        .collect();
    println!("max: {} min: {} ", max, min);
}
