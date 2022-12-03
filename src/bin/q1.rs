use advent_of_code::common::ReadChunks;
use itertools::Itertools;
use std::cmp::Reverse;
fn main() {
    let v: Vec<i32> = ReadChunks::new("./input1.txt")
        .map(|x| x.into_iter().map(|s| s.parse::<i32>().unwrap()).sum())
        .collect();
    println!("max: {}", v.iter().max().unwrap());
    println!(
        "top3: {}",
        v.iter().sorted_by_key(|x| Reverse(*x)).take(3).sum::<i32>()
    );
}
