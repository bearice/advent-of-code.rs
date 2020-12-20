use std::collections::VecDeque;

use advent_of_code::common::read_lines;

fn eval(s: String) -> usize {
    let s1 = s.replace("(", " ( ").replace(")", " ) ");
    let mut tkn = s1.split_ascii_whitespace().collect();
    eval_tokens(&mut tkn)
}
fn val(v: &mut VecDeque<&str>) -> usize {
    if let Some(x) = v.pop_front() {
        if x == "(" {
            eval_tokens(v)
        } else {
            x.parse().unwrap()
        }
    } else {
        panic!("no more tokens")
    }
}
fn eval_tokens1(v: &mut VecDeque<&str>) -> usize {
    // println!("evaling {:?}", v);
    let mut n1 = val(v);
    while let Some(tkn) = v.pop_front() {
        if tkn == ")" {
            break;
        }
        let op = tkn;
        let n2 = val(v);
        // println!("{} {} {}", n1, op, n2);
        n1 = match op {
            "+" => n1 + n2,
            "*" => n1 * n2,
            _ => panic!("not possible"),
        }
    }
    n1
}
fn eval_tokens(v: &mut VecDeque<&str>) -> usize {
    // println!("evaling {:?}", v);
    let mut vm = vec![];
    let mut n1 = val(v);
    while let Some(tkn) = v.pop_front() {
        if tkn == ")" {
            break;
        }
        let op = tkn;
        let n2 = val(v);
        // println!("{} {} {}", n1, op, n2);
        n1 = match op {
            "+" => n1 + n2,
            "*" => {
                vm.push(n1);
                n2
            }
            _ => panic!("not possible"),
        }
    }
    vm.push(n1);
    vm.iter().fold(1, |a, x| a * x)
}

fn main() {
    let n: usize = read_lines("./input18.txt")
        .unwrap()
        .map(|x| eval(x.unwrap()))
        .sum();
    println!("{}", n);
}
