use std::collections::{HashMap, HashSet};

use advent_of_code::common::ReadChunks;
use regex::Regex;

#[derive(Debug)]
struct Ranges {
    name: String,
    a: usize,
    b: usize,
    c: usize,
    d: usize,
}

impl Ranges {
    fn new(name: String, a: usize, b: usize, c: usize, d: usize) -> Self {
        Self { name, a, b, c, d }
    }
    fn contains(&self, n: usize) -> bool {
        (self.a <= n && self.b >= n) || (self.c <= n && self.d >= n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn range_contains() {
        let r = super::Ranges::new("test".to_owned(), 34, 100, 150, 200);
        assert!(r.contains(35));
        assert!(!r.contains(0));
    }
}

fn parse_ranges(lines: Vec<String>) -> Vec<Ranges> {
    let regex = Regex::new(r"([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    lines
        .iter()
        .map(|l| {
            let m = regex.captures_iter(l).next().unwrap();
            let name = m.get(1).unwrap().as_str();
            let a = m[2].parse::<usize>().unwrap();
            let b = m[3].parse::<usize>().unwrap();
            let c = m[4].parse::<usize>().unwrap();
            let d = m[5].parse::<usize>().unwrap();
            Ranges::new(name.to_owned(), a, b, c, d)
        })
        .collect()
}

fn parse_tickets(mut lines: Vec<String>) -> Vec<Vec<usize>> {
    lines.remove(0);
    lines
        .into_iter()
        .map(|s| {
            s.split(",")
                .map(str::parse::<usize>)
                .map(Result::unwrap)
                .collect()
        })
        .collect()
}

fn main() {
    let mut chunks = ReadChunks::new("./input16.txt");
    let ranges = parse_ranges(chunks.next().unwrap());
    let my_tkt = parse_tickets(chunks.next().unwrap());
    let tickets = parse_tickets(chunks.next().unwrap());
    let mut x = 0;
    // let mut valids = vec![];
    let names: HashSet<_> = ranges.iter().map(|x| &x.name).collect();
    let mut avail_names: Vec<_> = tickets[0].iter().map(|_| names.clone()).collect();

    for i in 0..tickets.len() {
        let mut c = 0;
        let t = &tickets[i];
        for j in 0..t.len() {
            let n = t[j];
            if !ranges.iter().any(|r| r.contains(n)) {
                x += n;
            } else {
                c += 1;
            }
        }
        if c == t.len() {
            for j in 0..t.len() {
                let n = t[j];
                for r in ranges.iter() {
                    // println!("{:?} {} {}", r, n, r.contains(n));
                    if avail_names[j].contains(&r.name) && !r.contains(n) {
                        // println!("{} removes {}", j, r.name);
                        avail_names[j].remove(&r.name);
                    }
                }
            }
        }
    }
    // This works, but looks very ugly :(
    println!("{}", x);
    let mut t: Vec<_> = avail_names.iter().enumerate().collect();
    t.sort_by_key(|x| x.1.len());
    let mut pos = HashMap::new();
    let mut done = HashSet::new();
    for i in 0..t.len() {
        // println!("{} = {:?}", i, t[i]);
        let d: HashSet<_> = t[i].1.difference(&done.clone()).cloned().collect();
        if d.len() == 1 {
            let name = d.iter().next().unwrap();
            pos.insert(*name, t[i].0);
            done.insert(*name);
        }
    }
    println!("{:?}", pos);
    let mut ret = 1;
    let t = &my_tkt[0];
    for p in pos.into_iter() {
        if p.0.starts_with("departure") {
            ret *= t[p.1];
        }
    }
    println!("{}", ret);
}
