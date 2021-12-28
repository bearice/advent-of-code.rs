use std::collections::HashMap;

use advent_of_code::common::{read_lines, shortest_path};
use itertools::Itertools;
fn main() {
    let map = read_lines("input20.txt")
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut thickness = 2;
    while ['.', '#'].contains(&map[thickness][thickness]) {
        thickness += 1;
    }
    // println!("Thickness: {}", thickness);
    let height = map.len();
    let width = map[0].len();
    let portals = find_portals(&map, thickness, height, width);
    let mut center = map[2..map.len() - 2]
        .iter()
        .map(|line| {
            line[2..line.len() - 2]
                .iter()
                .map(|&c| if c == '.' { 1 } else { 0 })
                .collect_vec()
        })
        .collect_vec();

    // println!("{:?}", portals);
    for (&id, &p) in portals.iter() {
        center[p.1][p.0] = id;
    }

    let start = portals[&name_to_id(('A', 'A'))];
    let end = portals[&name_to_id(('Z', 'Z'))];

    println!("start: {:?}", start);
    println!("end: {:?}", end);

    let shortest = shortest_path(start, end, |&p| next_move1(&center, &portals, p));
    println!("{:?}", shortest);

    let shortest = shortest_path((start, 0), (end, 0), |&p| next_move2(&center, &portals, p));
    println!("{:?}", shortest);
}

fn name_to_id((a, b): (char, char)) -> i32 {
    ((a as u32) << 8 | b as u32) as i32
}

fn find_portals(
    map: &[Vec<char>],
    thickness: usize,
    height: usize,
    width: usize,
) -> HashMap<i32, (usize, usize)> {
    let mut portals = HashMap::new();
    // outer portals
    let (left, right): (Vec<_>, Vec<_>) = map
        .iter()
        .map(|line| {
            let left = (line[0], line[1]);
            let right = (line[line.len() - 2], line[line.len() - 1]);
            (left, right)
        })
        .unzip();
    let top = map[0]
        .iter()
        .copied()
        .zip(map[1].iter().copied())
        .collect_vec();
    let bottom = map[height - 2]
        .iter()
        .copied()
        .zip(map[height - 1].iter().copied())
        .collect_vec();

    for (y, name) in left.into_iter().enumerate() {
        if name.0 == ' ' || name.1 == ' ' {
            continue;
        }
        let id = name_to_id(name);
        let pos = (0, y - 2);
        portals.insert(id, pos);
    }

    for (y, name) in right.into_iter().enumerate() {
        if name.0 == ' ' || name.1 == ' ' {
            continue;
        }
        let id = name_to_id(name);
        let pos = (width - 5, y - 2);
        portals.insert(id, pos);
    }

    for (x, name) in top.into_iter().enumerate() {
        if name.0 == ' ' || name.1 == ' ' {
            continue;
        }
        let id = name_to_id(name);
        let pos = (x - 2, 0);
        portals.insert(id, pos);
    }

    for (x, name) in bottom.into_iter().enumerate() {
        if name.0 == ' ' || name.1 == ' ' {
            continue;
        }
        let id = name_to_id(name);
        let pos = (x - 2, height - 5);
        portals.insert(id, pos);
    }

    // inner portals
    let top = map[thickness][thickness + 1..width - thickness - 1]
        .iter()
        .copied()
        .zip(
            map[thickness + 1][thickness + 1..width - thickness - 1]
                .iter()
                .copied(),
        )
        .collect_vec();

    let bottom = map[height - thickness - 2][thickness + 1..width - thickness - 1]
        .iter()
        .copied()
        .zip(
            map[height - thickness - 1][thickness + 1..width - thickness - 1]
                .iter()
                .copied(),
        )
        .collect_vec();

    let (left, right): (Vec<_>, Vec<_>) = map[thickness..height - thickness]
        .iter()
        .map(|line| {
            let left = (line[thickness], line[thickness + 1]);
            let right = (line[width - thickness - 2], line[width - thickness - 1]);
            (left, right)
        })
        .unzip();

    // println!("top: {:?}", top);
    // println!("bottom: {:?}", bottom);
    // println!("left: {:?}", left);
    // println!("right: {:?}", right);

    for (y, name) in left.into_iter().enumerate() {
        if name.0 == ' ' || name.1 == ' ' {
            continue;
        }
        let id = name_to_id(name);
        let pos = (thickness - 3, y + thickness - 2);
        portals.insert(-id, pos);
    }

    for (y, name) in right.into_iter().enumerate() {
        if name.0 == ' ' || name.1 == ' ' {
            continue;
        }
        let id = name_to_id(name);
        let pos = (width - thickness - 2, y + thickness - 2);
        portals.insert(-id, pos);
    }

    for (x, name) in top.into_iter().enumerate() {
        if name.0 == ' ' || name.1 == ' ' {
            continue;
        }
        let id = name_to_id(name);
        let pos = (x + thickness - 1, thickness - 3);
        portals.insert(-id, pos);
    }

    for (x, name) in bottom.into_iter().enumerate() {
        if name.0 == ' ' || name.1 == ' ' {
            continue;
        }
        let id = name_to_id(name);
        let pos = (x + thickness - 1, height - thickness - 2);
        portals.insert(-id, pos);
    }

    portals
}

fn next_move1(
    map: &[Vec<i32>],
    portals: &HashMap<i32, (usize, usize)>,
    (x, y): (usize, usize),
) -> Vec<((usize, usize), usize)> {
    let mut ret = vec![];
    if x > 0 && map[y][x - 1] != 0 {
        ret.push(((x - 1, y), 1));
    }
    if y > 0 && map[y - 1][x] != 0 {
        ret.push(((x, y - 1), 1));
    }
    if x < map[0].len() - 1 && map[y][x + 1] != 0 {
        ret.push(((x + 1, y), 1));
    }
    if y < map.len() - 1 && map[y + 1][x] != 0 {
        ret.push(((x, y + 1), 1));
    }
    if map[y][x] != 1 {
        let id = -map[y][x];
        // println!("id: {}", id);
        if let Some(p) = portals.get(&id) {
            ret.push((*p, 1));
        }
    }
    // println!("p: {},{} ret: {:?}", x, y, ret);
    ret
}

type Pos = (usize, usize);
fn next_move2(
    map: &[Vec<i32>],
    portals: &HashMap<i32, Pos>,
    ((x, y), level): (Pos, usize),
) -> Vec<((Pos, usize), usize)> {
    let mut ret = vec![];
    if x > 0 && map[y][x - 1] != 0 {
        ret.push((((x - 1, y), level), 1));
    }
    if y > 0 && map[y - 1][x] != 0 {
        ret.push((((x, y - 1), level), 1));
    }
    if x < map[0].len() - 1 && map[y][x + 1] != 0 {
        ret.push((((x + 1, y), level), 1));
    }
    if y < map.len() - 1 && map[y + 1][x] != 0 {
        ret.push((((x, y + 1), level), 1));
    }
    if map[y][x] != 1 {
        let id = map[y][x];
        // println!("id: {}", id);
        if level > 0 || id < 0 {
            let level = if id > 0 { level - 1 } else { level + 1 };
            let id = -id;
            if let Some(p) = portals.get(&id) {
                ret.push(((*p, level), 1));
            }
        }
    }
    // println!("p: ({},{})@{} ret: {:?}", x, y, level, ret);
    ret
}
