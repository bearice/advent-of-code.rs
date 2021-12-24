use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use advent_of_code::common::{find_edge, Program, ProgramOutput};

fn main() {
    let mut code = Program::from_file("input15.txt");

    let mut map = HashMap::new();
    let mut dirs = HashMap::new();
    let mut pos = (0, 0);
    let mut i = 0;
    let mut oxygen = (0, 0);
    while let Some(dir) = next_dir(&mut dirs, pos) {
        let out = match code.run_program(Some(dir + 1)) {
            ProgramOutput::Output(x) => x,
            // ProgramOutput::Halt => break,
            _ => panic!("Unexpected program output"),
        };
        // println!("pos: {:?} dir: {} out: {}", pos, dir, out);
        match out {
            0 => {
                let wall = next_pos(pos, dir);
                map.insert(wall, 0);
                // println!("wall: {:?}", wall);
                // dir += 1;
                // dir %= 4;
                // dir = next_dir(&mut dirs, pos, dir);
                // println!("new_dir: {}", dir);
            }
            1 => {
                pos = next_pos(pos, dir);
                map.insert(pos, 1);
                // println!("air: {:?}", pos);
            }
            2 => {
                pos = next_pos(pos, dir);
                map.insert(pos, 2);
                oxygen = pos;
                // println!("oxygen: {:?}", pos);
                // break;
            }
            x => panic!("Unexpected output: {}", x),
        }
        // if map.len() > 20 {
        //     break;
        // }
        // println!("new pos: {:?}", pos);
        if i % 10000 == 0 {
            print_map(&map, pos);
            println!("========");
        }
        i += 1;
    }
    // println!("{:?}", map);
    print_map(&map, (0, 0));

    let min = shortest_path((0, 0), oxygen, |pos| adj(&map, *pos));
    println!("min: {:?}", min);

    let mut time = 0;
    let mut queue = vec![oxygen];
    while map.values().any(|&x| x != 0 && x != 2) {
        let mut new_queue = vec![];
        for pos in queue {
            for dir in 0..4 {
                let next = next_pos(pos, dir);
                let ptr = map.entry(next).or_insert(0);
                if *ptr == 1 {
                    *ptr = 2;
                    new_queue.push(next);
                }
            }
        }
        queue = new_queue;
        time += 1;
    }
    println!("time: {}", time);
}

fn next_pos(pos: (i32, i32), dir: i64) -> (i32, i32) {
    match dir {
        0 => (pos.0, pos.1 - 1), // North
        1 => (pos.0, pos.1 + 1), // South
        2 => (pos.0 - 1, pos.1), // West
        3 => (pos.0 + 1, pos.1), // East
        _ => panic!("Invalid direction"),
    }
}

fn next_dir(cache: &mut HashMap<(i32, i32), (i32, i32)>, pos: (i32, i32)) -> Option<i64> {
    if !cache.is_empty() && cache.values().all(|&x| x.1 > 4) {
        return None;
    }
    let last = cache.entry(pos).or_insert((0, 0));
    last.0 = match last.0 {
        0 => 3,
        1 => 2,
        2 => 0,
        3 => 1,
        _ => panic!("Invalid direction"),
    };
    last.1 += 1;
    Some(last.0 as i64)
}

fn print_map(map: &HashMap<(i32, i32), i32>, pos: (i32, i32)) {
    let (min_x, max_x, min_y, max_y) = find_edge(map.keys().cloned());
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if x == 0 && y == 0 {
                print!("O");
            } else if x == pos.0 && y == pos.1 {
                print!("D");
            } else {
                match map.get(&(x, y)) {
                    Some(0) => print!("w"),
                    Some(1) => print!("."),
                    Some(2) => print!("*"),
                    _ => print!("?"),
                }
            }
        }
        println!();
    }
}

fn adj(map: &HashMap<(i32, i32), i32>, pos: (i32, i32)) -> Vec<((i32, i32), usize)> {
    let mut adj = Vec::new();
    for dir in 0..4 {
        let next = next_pos(pos, dir as i64);
        if let Some(val) = map.get(&next) {
            if *val != 0 {
                adj.push((next, 1));
            }
        }
    }
    adj
}

fn shortest_path<T, F>(start: T, end: T, edges: F) -> Option<usize>
where
    T: std::hash::Hash + Eq + Clone + Copy + Ord,
    F: Fn(&T) -> Vec<(T, usize)>,
{
    let mut dist: HashMap<T, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(Reverse((0, start)));

    while let Some(Reverse((cost, pos))) = heap.pop() {
        if pos == end {
            return Some(cost);
        }
        if cost > dist[&pos] {
            continue;
        }
        for (edge, new_cost) in edges(&pos) {
            let new_cost = cost + new_cost;
            let d = dist.entry(edge).or_insert(usize::MAX);
            if new_cost < *d {
                heap.push(Reverse((new_cost, edge)));
                *d = new_cost;
            }
        }
    }
    None
}
