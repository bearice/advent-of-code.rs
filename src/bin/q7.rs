use std::collections::{HashMap, HashSet};

use advent_of_code::common::read_lines;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    //example: light red bags contain 1 bright white bags, 2 muted yellow bags.
    static ref PARSER: Regex = {
        Regex::new(
        r"(?x)
            (\w+\ \w+)\ bags\ contain
            |
            (?:\ (\d)\ (\w+\ \w+)\ bags?[,\.]?)
            |
            \ no\ other\ bags\.
        "
        ).unwrap()
    };
}

fn parse_line(line: &String) -> (String, Vec<(String, usize)>) {
    let matches = PARSER.captures_iter(line);
    let mut name = "";
    let mut value: Vec<(String, usize)> = vec![];
    for m in matches {
        match (m.get(1), m.get(2), m.get(3)) {
            (Some(n), None, None) => name = n.as_str(),
            (None, Some(cnt), Some(clr)) => {
                value.push((clr.as_str().to_owned(), cnt.as_str().parse().unwrap()))
            }
            (None, None, None) => (),
            _ => panic!("not possible"),
        }
    }
    (name.to_owned(), value)
}

type BagMap = HashMap<String, Vec<(String, usize)>>;

fn search1(map: &BagMap, name: &String) -> HashSet<String> {
    if let Some(vec) = map.get(name) {
        // println!("{} => {:?}", name, vec);
        let ret = vec.iter().map(|(name, _)| name).cloned().collect();
        vec.iter()
            .map(|(name, _)| search1(map, name))
            .fold(ret, |acc, val| acc.union(&val).cloned().collect())
    } else {
        // println!("{} => None", name);
        HashSet::new()
    }
}

fn search2(map: &BagMap, name: &String) -> usize {
    let vec = map.get(name).unwrap();
    if !vec.is_empty() {
        // println!("{} => {:?}", name, vec);
        vec.iter()
            .map(|(name, cnt)| cnt * search2(map, name))
            .sum::<usize>()
            + 1
    } else {
        // println!("{} => None", name);
        1
    }
}

fn main() {
    let root = "shiny gold".to_owned();
    let lines = read_lines("./input7.txt");
    let rules: BagMap = lines.map(|line| parse_line(&line)).collect();
    let mut rev_rules: BagMap = HashMap::new();

    for (k, v) in rules.iter() {
        for (n, i) in v {
            if let Some(vec) = rev_rules.get_mut(n) {
                vec.push((k.to_owned(), *i));
            } else {
                rev_rules.insert(n.to_owned(), vec![(k.to_owned(), *i)]);
            }
        }
    }
    // println!("{:?}", rules);
    // println!("{:?}", rev_rules);
    println!("q1={}", search1(&rev_rules, &root).len());
    println!("q2={}", search2(&rules, &root) - 1);
}
