use std::collections::HashSet;

use advent_of_code::common::read_lines;

fn validate(v: &[usize], n: usize) -> bool {
    //println!("v={:?} n={}", v, n);
    let r: HashSet<usize> = v
        .iter()
        .filter(|x| **x < n && **x != n / 2)
        .map(|x| n - x)
        .collect();
    r.is_disjoint(&v.iter().cloned().collect())
}
fn main() {
    let numbers: Vec<usize> = read_lines("./input9.txt")
        .unwrap()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();

    let window = 25;
    let mut n = 0;
    for i in window..numbers.len() {
        n = numbers[i];
        if validate(&numbers[i - window..i], n) {
            println!("{}", n);
            break;
        }
    }
    let (mut a, mut b) = (0, 1);
    while b < numbers.len() {
        let s = &numbers[a..=b];
        let sum = (*s).iter().sum::<usize>();
        if sum == n {
            let min = (*s).iter().min().unwrap();
            let max = (*s).iter().max().unwrap();
            println!("{}", min + max);
            break;
        } else if sum < n {
            b += 1;
        } else {
            a += 1;
        }
    }
}
