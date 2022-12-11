use std::{cmp::Reverse, mem::swap, rc::Rc};

use advent_of_code::common::ReadChunks;
use itertools::Itertools;

fn main() {
    let monkeys = ReadChunks::new("input11.txt")
        .map(parse_monkey)
        .collect_vec();

    solve(monkeys.clone(), 20, 3);
    solve(monkeys, 10000, 1);
}
fn solve(mut monkeys: Vec<Monkey>, rounds: usize, div_by: usize) {
    let mut mod_by = 1;
    for m in &monkeys {
        mod_by *= m.test;
    }
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for (new, next) in monkeys[i].take_turn(div_by, mod_by) {
                monkeys[next].items.push(new);
            }
        }
    }
    monkeys.sort_by_key(|x| Reverse(x.inspects));
    println!("{}", monkeys[0].inspects * monkeys[1].inspects);
}

#[allow(dead_code)]
#[derive(Clone)]
struct Monkey {
    id: usize,
    inspects: usize,
    items: Vec<usize>,
    op: Rc<dyn Fn(usize) -> usize>,
    test: usize,
    target: (usize, usize),
}

impl Monkey {
    fn take_turn(&mut self, div_by: usize, mod_by: usize) -> Vec<(usize, usize)> {
        let mut items = vec![];
        swap(&mut self.items, &mut items);
        self.inspects += items.len();
        items
            .into_iter()
            .map(|i| {
                // println!("Monkey {} inpsect item {}", self.id, i);
                let new = (((self.op)(i)) / div_by) % mod_by;
                // println!("New item: {}", new);
                let next = if new % self.test == 0 {
                    self.target.0
                } else {
                    self.target.1
                };
                (new, next)
            })
            .collect_vec()
    }
}
fn parse_monkey(input: Vec<String>) -> Monkey {
    let id = input[0][7..8].parse().unwrap();
    let items = parse_items(&input[1]);
    let op = parse_op(&input[2]);
    let test = parse_test(&input[3]);
    let target = parse_target(&input[4], &input[5]);
    Monkey {
        id,
        inspects: 0,
        items,
        op,
        test,
        target,
    }
}

//Starting items: 54, 65, 75, 74
fn parse_items(input: &str) -> Vec<usize> {
    input
        .split_once(':')
        .unwrap()
        .1
        .split(',')
        .map(str::trim)
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

//Operation: new = old * 19
fn parse_op(input: &str) -> Rc<dyn Fn(usize) -> usize> {
    let expr = input.split_once('=').unwrap().1;
    let tokens = expr.split_ascii_whitespace().collect::<Vec<_>>();
    let op1 = tokens[0].parse();
    let op2 = tokens[2].parse();
    let op = tokens[1].to_owned();
    let ret = move |old| {
        let op1 = op1.clone().unwrap_or(old);
        let op2 = op2.clone().unwrap_or(old);
        match op.as_str() {
            "+" => op1 + op2,
            "-" => op1 - op2,
            "*" => op1 * op2,
            "/" => op1 / op2,
            _ => unreachable!(),
        }
    };
    Rc::new(ret)
}

//Test: divisible by 13
fn parse_test(input: &str) -> usize {
    input.split_once("by ").unwrap().1.parse().unwrap()
}

//If true: throw to monkey 1
//If false: throw to monkey 3
fn parse_target(if_true: &str, if_false: &str) -> (usize, usize) {
    let if_true = if_true.split_once("monkey ").unwrap().1.parse().unwrap();
    let if_false = if_false.split_once("monkey ").unwrap().1.parse().unwrap();
    (if_true, if_false)
}
