use advent_of_code::common::read_lines;
use itertools::Itertools;

fn main() {
    let parts = read_lines("./input4.txt").map(parse_line).collect_vec();
    let a1 = parts.iter().filter(contains).count();
    println!("{}", a1);
    let a2 = parts.iter().filter(overlap).count();
    println!("{}", a2);
}

fn parse_line(line: String) -> ((i32, i32), (i32, i32)) {
    let mut parts = line.split([',', '-']).map(str::parse).map(Result::unwrap);
    let a = parts.next_tuple().unwrap();
    let b = parts.next_tuple().unwrap();
    (a, b)
}

fn contains(((a, b), (c, d)): &&((i32, i32), (i32, i32))) -> bool {
    (a <= c && b >= d) || (a >= c && b <= d)
}

fn overlap(((a, b), (c, d)): &&((i32, i32), (i32, i32))) -> bool {
    a <= d && c <= b
}
