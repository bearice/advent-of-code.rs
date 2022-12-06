use std::{collections::HashSet, iter::FromIterator};

use advent_of_code::common::read_lines;
use itertools::Itertools;

fn main() {
    let input = read_lines("./input6.txt")
        .next()
        .unwrap()
        .chars()
        .collect_vec();

    println!("{:?}", find_marker(&input, 4));
    println!("{:?}", find_marker(&input, 14));
}

fn find_marker(input: &[char], size: usize) -> Option<usize> {
    let mut n = size;
    for slice in input.windows(size) {
        if HashSet::<char>::from_iter(slice.iter().cloned()).len() == size {
            return Some(n);
        }
        n += 1;
    }
    None
}
