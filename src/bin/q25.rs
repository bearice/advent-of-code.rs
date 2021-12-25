use std::collections::HashMap;

use advent_of_code::common::read_lines;
type Pos = (usize, usize);
fn main() {
    let mut map = HashMap::new();
    let lines = read_lines("input25.txt");
    for (y, line) in lines.enumerate() {
        for (x, c) in line.char_indices() {
            map.insert((x, y), c);
        }
    }

    let edge = map
        .keys()
        .copied()
        .reduce(|(x, y), (x2, y2)| (x.max(x2), y.max(y2)))
        .map(|(x, y)| (x + 1, y + 1))
        .unwrap();

    let mut i = 1;
    loop {
        let moved = step(&mut map, edge);
        println!("step {} moved {}", i, moved);
        if moved == 0 {
            break;
        }
        i += 1;
    }
    // print_map(&map, edge);
    println!("{}", i);
}

fn step(map: &mut HashMap<Pos, char>, edge: Pos) -> usize {
    let mut moved = 0;
    let mut new_map = map.clone();
    for (&pos, &c) in map.iter() {
        if c == '>' {
            let next = ((pos.0 + 1) % edge.0, pos.1);
            if map[&next] == '.' {
                new_map.insert(next, c);
                new_map.insert(pos, '.');
                moved += 1;
            }
        }
    }
    *map = new_map.clone();
    for (&pos, &c) in map.iter() {
        if c == 'v' {
            let next = (pos.0, (pos.1 + 1) % edge.1);
            if map[&next] == '.' {
                new_map.insert(next, c);
                new_map.insert(pos, '.');
                moved += 1;
            }
        }
    }
    *map = new_map;
    moved
}

#[allow(dead_code)]
fn print_map(map: &HashMap<Pos, char>, edge: Pos) {
    for y in 0..edge.1 {
        for x in 0..edge.0 {
            let c = map.get(&(x, y)).unwrap_or(&'?');
            print!("{}", c);
        }
        println!();
    }
}
