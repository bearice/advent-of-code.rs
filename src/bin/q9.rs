use std::{cmp::Reverse, collections::HashSet};

use advent_of_code::common::read_lines;

fn main() {
    let numbers = read_lines("./input9.txt")
        .map(|x| {
            x.into_bytes()
                .into_iter()
                .map(|y| (y - 48) as u32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // let mut sum = 0;
    let mut lowest = Vec::new();
    for (y, row) in numbers.iter().enumerate() {
        for (x, i) in row.iter().enumerate() {
            let neighbors = neighbors(x, y, &numbers);
            if neighbors.iter().all(|&n| n.0 > *i) {
                lowest.push((*i, x, y));
            }
        }
    }
    println!("{}", lowest.iter().map(|&n| n.0 + 1).sum::<u32>());
    let mut basins = Vec::new();
    for p in lowest {
        basins.push(find_basin(p, &numbers));
    }
    basins.sort_by_key(|x| Reverse(*x));
    // println!("{:?}", basins);
    println!("{}", basins[0] * basins[1] * basins[2]);
}

fn find_basin((i, x, y): (u32, usize, usize), numbers: &[Vec<u32>]) -> usize {
    let mut basin = HashSet::new();
    let mut visited = HashSet::new();
    let mut queue = vec![(i, x, y)];
    while let Some(pos) = queue.pop() {
        // println!("pos={:?}", pos);
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        for n in neighbors(pos.1, pos.2, numbers) {
            // println!("n={:?}", n);
            if n.0 == 9 || n.0 < pos.0 {
                continue;
            }
            if !visited.contains(&n) {
                queue.push(n);
            }
            basin.insert(n);
        }
    }
    // println!("basin={:?}", basin);
    basin.len() + 1
}

fn neighbors(x: usize, y: usize, numbers: &[Vec<u32>]) -> Vec<(u32, usize, usize)> {
    let mut ret = Vec::new();
    let mut push_number = |x: usize, y: usize| ret.push((numbers[y][x], x, y));
    if x > 0 {
        push_number(x - 1, y);
    }
    if y > 0 {
        push_number(x, y - 1);
    }
    if x < numbers[y].len() - 1 {
        push_number(x + 1, y);
    }
    if y < numbers.len() - 1 {
        push_number(x, y + 1);
    }
    ret
}
