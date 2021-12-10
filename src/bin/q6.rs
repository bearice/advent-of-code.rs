use std::collections::HashMap;

use advent_of_code::common::read_lines;

fn main() {
    let input = read_lines("input6.txt");
    let mut map = HashMap::new();
    for line in input {
        let (parent, node) = line.split_once(')').unwrap();
        map.insert(node.to_string(), parent.to_string());
    }
    let mut count = 0;
    for key in map.keys() {
        count += find_path(&map, key).len();
    }
    println!("{}", count);

    let p1 = find_path(&map, "YOU");
    let p2 = find_path(&map, "SAN");
    let mut i = 0;
    while p1[i] == p2[i] {
        i += 1;
    }
    println!("{}", p1.len() + p2.len() - 2 * i);
}

fn find_path<'a>(map: &'a HashMap<String, String>, start: &str) -> Vec<&'a str> {
    let mut path = Vec::new();
    let mut current = start;
    while let Some(parent) = map.get(current) {
        path.push(parent.as_str());
        current = parent;
    }
    path.reverse();
    path
}
