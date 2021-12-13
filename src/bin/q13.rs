use std::collections::HashSet;

use advent_of_code::common::ReadChunks;

fn main() {
    let mut chunks = ReadChunks::new("input13.txt");
    let mut map = chunks
        .next()
        .unwrap()
        .into_iter()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let ops = chunks.next().unwrap().into_iter().map(|line| {
        let (dir, pos) = line.split_once('=').unwrap();
        (dir.chars().last().unwrap(), pos.parse().unwrap())
    });

    for (dir, pos) in ops {
        map = match dir {
            'x' => fold_x(map, pos),
            'y' => fold_y(map, pos),
            _ => panic!("Unknown direction"),
        };
        println!("dir={} pos={} len={}", dir, pos, map.len());
    }
    print_map(map);
}

fn fold_y(v: HashSet<(i32, i32)>, pos: i32) -> HashSet<(i32, i32)> {
    v.into_iter()
        .map(|(x, y)| (x, if y <= pos { y } else { pos * 2 - y }))
        .collect()
}

fn fold_x(v: HashSet<(i32, i32)>, pos: i32) -> HashSet<(i32, i32)> {
    v.into_iter()
        .map(|(x, y)| (if x <= pos { x } else { pos * 2 - x }, y))
        .collect()
}

fn print_map(v: HashSet<(i32, i32)>) {
    let mut min_x = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut min_y = std::i32::MAX;
    let mut max_y = std::i32::MIN;
    for &(x, y) in v.iter() {
        min_x = std::cmp::min(min_x, x);
        max_x = std::cmp::max(max_x, x);
        min_y = std::cmp::min(min_y, y);
        max_y = std::cmp::max(max_y, y);
    }
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if v.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
