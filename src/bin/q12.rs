use std::collections::HashMap;

use advent_of_code::common::read_lines;

fn main() {
    let lines: Vec<String> = read_lines("./input12.txt").collect();
    let mut adjs = HashMap::new();
    for line in lines {
        let (a, b) = line.split_once('-').unwrap();
        adjs.entry(a.to_owned())
            .or_insert_with(Vec::new)
            .push(b.to_owned());
        adjs.entry(b.to_owned())
            .or_insert_with(Vec::new)
            .push(a.to_owned());
    }
    let paths = path_to_end("start", &adjs, &mut HashMap::new(), filter1);
    println!("{}", paths);
    let paths = path_to_end("start", &adjs, &mut HashMap::new(), filter2);
    println!("{}", paths);
}

fn filter1(node: &str, visted: &HashMap<String, u32>) -> bool {
    let is_small_cave = node.chars().all(char::is_lowercase);
    let count = visted.get(node).unwrap_or(&0);
    !(is_small_cave && *count > 0)
}

fn filter2(node: &str, visited: &HashMap<String, u32>) -> bool {
    let count = visited.get(node);
    if node == "start" {
        count.map(|x| *x == 0).unwrap_or(true)
    } else if node.chars().all(char::is_lowercase) {
        if count.map(|x| *x > 0).unwrap_or(false) {
            !visited
                .iter()
                .any(|c| c.0.chars().all(char::is_lowercase) && *c.1 > 1)
        } else {
            true
        }
    } else {
        true
    }
}

fn path_to_end<F>(
    start: &str,
    adjs: &HashMap<String, Vec<String>>,
    visited: &mut HashMap<String, u32>,
    filter: F,
) -> u32
where
    F: Fn(&str, &HashMap<String, u32>) -> bool + Copy,
{
    if start == "end" {
        return 1;
    }
    let mut paths = 0;
    if !filter(start, visited) {
        return 0;
    }
    *visited.entry(start.to_owned()).or_insert(0) += 1;
    for adj in adjs.get(start).unwrap() {
        paths += path_to_end(adj, adjs, visited, filter);
    }
    *visited.entry(start.to_owned()).or_insert(0) -= 1;
    paths
}
