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
    let paths = path_to_end("start", &adjs, HashMap::new(), filter1);
    println!("{}", paths.len());
    let paths = path_to_end("start", &adjs, HashMap::new(), filter2);
    println!("{}", paths.len());
}

fn filter1(node: &str, visted: &HashMap<String, u32>) -> bool {
    let is_small_cave = node.chars().all(char::is_lowercase);
    let count = visted.get(node).unwrap_or(&0);
    !(is_small_cave && *count > 0)
}

fn filter2(node: &str, visited: &HashMap<String, u32>) -> bool {
    let count = visited.get(node);
    if node == "start" {
        count.is_none()
    } else if node.chars().all(char::is_lowercase) {
        if count.is_some() {
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
    mut visited: HashMap<String, u32>,
    filter: F,
) -> Vec<Vec<String>>
where
    F: Fn(&str, &HashMap<String, u32>) -> bool + Copy,
{
    if start == "end" {
        return vec![vec![start.to_owned()]];
    }
    let mut paths = Vec::new();
    if !filter(start, &visited) {
        return vec![];
    }
    let count = visited.entry(start.to_owned()).or_insert(0);
    *count += 1;
    for adj in adjs.get(start).unwrap() {
        let new_paths = path_to_end(adj, adjs, visited.clone(), filter);
        for mut path in new_paths {
            path.push(start.to_owned());
            paths.push(path);
        }
    }
    paths
}
