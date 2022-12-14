use std::{collections::HashSet, num::ParseIntError, str::FromStr};

use advent_of_code::common::read_lines;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn try_move(&self) -> [Point; 3] {
        [
            Self {
                x: self.x,
                y: self.y + 1,
            },
            Self {
                x: self.x - 1,
                y: self.y + 1,
            },
            Self {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }
}

impl FromStr for Point {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Point {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

fn parse_line(s: String, map: &mut HashSet<Point>) {
    s.split("->")
        .map(|s| s.trim().parse().unwrap())
        .tuple_windows()
        .for_each(|(from, to)| add_path(from, to, map));
}

fn add_path(from: Point, to: Point, map: &mut HashSet<Point>) {
    // println!("add_path: {:?} -> {:?}", from, to);
    if from.x == to.x {
        for y in from.y.min(to.y)..from.y.max(to.y) + 1 {
            map.insert(Point { x: from.x, y });
        }
    } else if from.y == to.y {
        for x in from.x.min(to.x)..from.x.max(to.x) + 1 {
            map.insert(Point { x, y: from.y });
        }
    } else {
        unreachable!()
    }
}

fn add_sand(origin: Point, limit: i32, bottom: Option<i32>, map: &mut HashSet<Point>) -> bool {
    if map.contains(&origin) {
        return false;
    }
    let mut new = origin;
    while new.y < limit {
        let mut next = None;
        for p in new.try_move() {
            if map.contains(&p) || Some(p.y) == bottom {
                continue;
            }
            next = Some(p);
            break;
        }
        if let Some(next) = next {
            new = next;
        } else {
            map.insert(new);
            return true;
        }
    }
    false
}

fn main() {
    let mut map = HashSet::new();
    read_lines("input14.txt").for_each(|line| parse_line(line, &mut map));
    let limit = map.iter().max_by_key(|p| p.y).unwrap().y;
    {
        let mut map = map.clone();
        let origin = Point::new(500, 0);
        let mut count = 0;
        while add_sand(origin, limit, None, &mut map) {
            count += 1;
        }
        println!("{}", count);
    }
    {
        let mut map = map;
        let origin = Point::new(500, 0);
        let mut count = 0;
        while add_sand(origin, limit + 3, Some(limit + 2), &mut map) {
            count += 1;
        }
        println!("{}", count);
    }
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
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if v.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
