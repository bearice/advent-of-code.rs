use advent_of_code::common::read_lines;
use itertools::Itertools;

fn main() {
    let mut lines = read_lines("./input5.txt");
    let mut stack = vec![];
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        stack.push(line.chars().collect_vec());
    }
    let stack = parse_stack(stack);
    // println!("Stack: {:?}", stack);
    let commands = lines.map(parse_cmd).collect_vec();
    // println!("Commands: {:?}", commands);
    {
        let mut stack = stack.clone();
        for cmd in &commands {
            exec_cmd1(&mut stack, *cmd)
        }
        print_stack(&stack);
    }
    {
        let mut stack = stack;
        for cmd in &commands {
            exec_cmd2(&mut stack, *cmd)
        }
        print_stack(&stack);
    }
}

fn parse_stack(mut stack: Vec<Vec<char>>) -> Vec<Vec<char>> {
    stack.reverse();
    let indexes = &stack[0];
    let mut ret = vec![vec![]; 11];
    for (ch, idx) in indexes.iter().zip(0usize..) {
        if ch.is_numeric() {
            let n = ch.to_digit(10).unwrap() as usize;
            for row in &stack[1..] {
                if row[idx].is_ascii_alphabetic() {
                    ret[n].push(row[idx]);
                }
            }
        }
    }
    ret
}

fn parse_cmd(s: String) -> (usize, usize, usize) {
    let substrings = s.split_ascii_whitespace().collect_vec();
    let count = substrings[1].parse().unwrap();
    let from = substrings[3].parse().unwrap();
    let to = substrings[5].parse().unwrap();
    (count, from, to)
}

fn exec_cmd1(stack: &mut [Vec<char>], (count, from, to): (usize, usize, usize)) {
    for _ in 0..count {
        let x = stack[from].pop().unwrap();
        stack[to].push(x);
    }
}

fn exec_cmd2(stack: &mut [Vec<char>], (count, from, to): (usize, usize, usize)) {
    let n = stack[from].len();
    let x = stack[from].splice(n - count..n, []).collect_vec();
    stack[to].extend(x);
}

fn print_stack(stack: &[Vec<char>]) {
    println!(
        "{}",
        stack
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| x[x.len() - 1])
            .join("")
    );
}
