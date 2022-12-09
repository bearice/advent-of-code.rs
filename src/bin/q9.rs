use std::{collections::HashSet, fmt::Debug};

use advent_of_code::common::read_lines;

#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn main() {
    let input = read_lines("./input9.txt")
        .map(|x| {
            let mut i = x.split_ascii_whitespace();
            (
                i.next().unwrap().chars().next().unwrap(),
                i.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    println!("{}", move_rope(2, &input));
    println!("{}", move_rope(10, &input));
}

fn move_rope(len: usize, steps: &[(char, i32)]) -> usize {
    let mut rope: Vec<Point> = vec![Default::default(); len];
    let mut visited = HashSet::new();
    for &(dir, count) in steps {
        for _ in 0..count {
            match dir {
                'U' => rope[0].y += 1,
                'D' => rope[0].y -= 1,
                'L' => rope[0].x -= 1,
                'R' => rope[0].x += 1,
                _ => unreachable!(),
            }
            for i in 0..len - 1 {
                rope[i + 1] = move_tail(rope[i], rope[i + 1]);
            }
            visited.insert(rope[len - 1]);
        }
    }
    // print_map(&visited);
    visited.len()
}

fn move_tail(head: Point, tail: Point) -> Point {
    let dx = tail.x - head.x;
    let dy = tail.y - head.y;
    // print!(" dx={} dy={}", dx, dy);
    if dx.abs() == 2 || dy.abs() == 2 {
        return Point {
            x: head.x + dx / 2,
            y: head.y + dy / 2,
        };
    }
    tail
}

#[allow(dead_code)]
fn print_map(v: &HashSet<Point>) {
    let mut min_x = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut min_y = std::i32::MAX;
    let mut max_y = std::i32::MIN;
    for &p in v {
        min_x = std::cmp::min(min_x, p.x);
        max_x = std::cmp::max(max_x, p.x);
        min_y = std::cmp::min(min_y, p.y);
        max_y = std::cmp::max(max_y, p.y);
    }
    for y in -max_y..=-min_y {
        for x in min_x..=max_x {
            let y = -y;
            if x == 0 && y == 0 {
                print!("s");
            } else if v.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
