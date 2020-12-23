use std::collections::{HashSet, VecDeque};

use advent_of_code::common::read_lines;

fn read_numbers(i: &mut dyn Iterator<Item = String>) -> VecDeque<usize> {
    let mut ret = VecDeque::new();
    while let Some(n) = i.next() {
        if n == "" {
            break;
        }
        if !n.starts_with("P") {
            ret.push_back(n.parse::<usize>().unwrap());
        }
    }
    ret
}

fn play1(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> VecDeque<usize> {
    while p1.len() > 0 && p2.len() > 0 {
        let n1 = p1.pop_front().unwrap();
        let n2 = p2.pop_front().unwrap();
        // println!("p1={:?} p2={:?} n1={} n2={}", p1.len(), p2.len(), n1, n2);
        assert!(n1 != n2);
        if n1 > n2 {
            p1.push_back(n1);
            p1.push_back(n2);
        } else {
            p2.push_back(n2);
            p2.push_back(n1);
        }
    }
    if p1.len() > 0 {
        p1
    } else {
        p2
    }
}

fn play2(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> (bool, VecDeque<usize>) {
    let mut records = HashSet::new();
    while p1.len() > 0 && p2.len() > 0 {
        let n1 = p1.pop_front().unwrap();
        let n2 = p2.pop_front().unwrap();
        if !records.insert(p1.clone()) && !records.insert(p2.clone()) {
            // println!("p1 wins early");
            return (true, p1);
        }
        let win = if n1 <= p1.len() && n2 <= p2.len() {
            // println!("p1={:?} p2={:?} n1={} n2={}", p1.len(), p2.len(), n1, n2);
            // println!("recursive play");
            let mut x1 = p1.clone();
            x1.resize(n1, 0);
            let mut x2 = p2.clone();
            x2.resize(n2, 0);
            play2(x1, x2).0
        } else {
            n1 > n2
        };
        if win {
            // println!("p1 wins");
            p1.push_back(n1);
            p1.push_back(n2);
        } else {
            // println!("p2 wins");
            p2.push_back(n2);
            p2.push_back(n1);
        }
    }
    if p1.len() > 0 {
        (true, p1)
    } else {
        (false, p2)
    }
}

fn score(winner: &VecDeque<usize>) -> usize {
    println!("{:?}", winner);
    let len = winner.len();
    winner
        .iter()
        .enumerate()
        .fold(0, |acc, x| acc + (len - x.0) * *x.1)
}

fn main() {
    let mut lines = read_lines("./input22.txt").unwrap().map(Result::unwrap);
    let p1 = read_numbers(&mut lines);
    let p2 = read_numbers(&mut lines);

    let winner1 = play1(p1.clone(), p2.clone());
    println!("{}", score(&winner1));

    let (_, winner2) = play2(p1.clone(), p2.clone());
    println!("{}", score(&winner2));
}
