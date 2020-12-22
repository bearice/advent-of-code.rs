use std::collections::{HashMap, HashSet};

use advent_of_code::common::read_lines;

fn parse_line(line: String) -> (HashSet<String>, HashSet<String>) {
    let mut ret = (HashSet::new(), HashSet::new());
    let mut words = line.split_ascii_whitespace();
    while let Some(word) = words.next() {
        if word == "(contains" {
            break;
        } else {
            ret.0.insert(word.to_owned());
        }
    }
    while let Some(word) = words.next() {
        ret.1.insert(word[..word.len() - 1].to_owned());
    }
    ret
}

fn main() {
    let lines: Vec<_> = read_lines("./input21.txt")
        .unwrap()
        .map(|x| parse_line(x.unwrap()))
        .collect();
    // println!("{:?}", lines);
    let mut words: HashMap<String, HashSet<String>> = HashMap::new();
    for i in 0..lines.len() {
        let l1 = &lines[i];
        for w in l1.1.iter() {
            if let Some(x) = words.get_mut(w) {
                *x = x.intersection(&l1.0).cloned().collect();
            } else {
                words.insert(w.to_owned(), l1.0.clone());
            }
        }
    }
    let keys: Vec<String> = words.keys().cloned().collect();
    let mut known_words = HashMap::new();
    while known_words.len() < keys.len() {
        for k in keys.iter() {
            if let Some(w) = words.get_mut(k) {
                if w.len() == 1 {
                    known_words.insert(k.to_owned(), w.iter().cloned().next().unwrap());
                } else {
                    for kw in known_words.values() {
                        w.remove(kw);
                    }
                }
            }
        }
    }
    // println!("{:?}", known_words);
    let mut cnt = 0;
    let kw: HashSet<_> = known_words.values().cloned().collect();
    for line in lines {
        cnt += line.0.difference(&kw).count();
    }
    println!("{:?}", cnt);

    let mut vw: Vec<_> = known_words.keys().collect();
    vw.sort();

    let mut ret = "".to_owned();
    for w in vw {
        ret += &known_words[w];
        ret.push(',');
    }
    println!("{:?}", ret)
}
