use advent_of_code::common::read_lines;
use std::collections::HashSet;

fn main() {
    let lines = read_lines("./input6.txt");
    let mut cnt = 0;
    let mut buf: Option<HashSet<char>> = None;
    for line in lines {
        if line.len() > 0 {
            let a = line.chars().collect();
            if let Some(b) = buf {
                buf = Some(b.intersection(&a).cloned().collect());
            } else {
                buf = Some(a);
            }
        } else {
            let b = buf.unwrap_or_default();
            println!("{}", b.len());
            cnt += b.len();
            buf = None;
        }
    }
    println!("{}", cnt);
}
