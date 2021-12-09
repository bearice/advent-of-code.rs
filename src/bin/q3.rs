use std::ops::{self, Not, Shl, Shr};

use advent_of_code::common::read_lines;

fn main() {
    let input = read_lines("input3.txt");
    let lines = input.map(|line| {
        line.split(',')
            .map(|s| (s.chars().next().unwrap(), s[1..].parse::<i32>().unwrap()))
            .collect::<Vec<_>>()
    });
    let points = lines.map(|line| {
        line.iter()
            .fold(Vec::new(), |(x, y), (dir, dist)| match dir {
                'R' => (x + dist, y),
                'L' => (x - dist, y),
                'U' => (x, y + dist),
                'D' => (x, y - dist),
                _ => panic!("invalid direction"),
            })
    });
}
