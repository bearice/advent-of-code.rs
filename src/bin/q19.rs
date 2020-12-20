use advent_of_code::common::read_lines;
use regex::Regex;

// extern crate pest;
// #[macro_use]
// extern crate pest_derive;
// use pest::Parser;

// #[derive(Parser)]
// #[grammar = "../input19-a2.pest"] // relative to src
// struct MyParser;

fn gen_regex(v: &Vec<(usize, &str)>, s: &str) -> String {
    let l = s.split_ascii_whitespace();
    let r: String = l
        .map(|i| {
            if let Ok(n) = i.parse::<usize>() {
                let x = v[n].1;
                gen_regex(v, x)
            } else {
                i.replace("\"", "").to_owned()
            }
        })
        .collect();
    if r.contains("|") {
        format!("({})", r)
    } else {
        r
    }
}
fn main() {
    let lines: Vec<_> = read_lines("./input19-a1.txt")
        .unwrap()
        .map(|x| x.unwrap())
        .collect();
    let mut l: Vec<_> = lines
        .iter()
        .map(|x| {
            let mut i = x.split(":");
            let idx: usize = i.next().unwrap().parse().unwrap();
            let cnt = i.next().unwrap();
            (idx, cnt)
        })
        .collect();
    l.sort();
    assert!(l.first().unwrap().0 == 0);
    assert!(l.last().unwrap().0 == l.len() - 1);

    let mut r = gen_regex(&l, l[0].1);
    r = format!("^{}$", r);
    let regex = Regex::new(&r).unwrap();
    let lines: Vec<_> = read_lines("./input19-b.txt")
        .unwrap()
        .map(Result::unwrap)
        .collect();
    let p1: Vec<_> = lines
        .iter()
        .map(|s| regex.is_match(&s))
        .filter(|x| *x)
        .collect();
    println!("p1={}", p1.len());
    // let p2: Vec<_> = lines
    //     .iter()
    //     .map(|s| MyParser::parse(Rule::p0, s))
    //     .filter(|x| !x.is_ok())
    //     .collect();
    // println!("p2={:?}", p2.first());
    // println!("p2={:?}", p2.len());
    //0: 8 11 -> 42+ (?<pn> 42 (?&pn) 31)
    //8: 42 | 42 8
    //11: 42 31 | 42 11 31
    let r42 = gen_regex(&l, l[42].1);
    let r31 = gen_regex(&l, l[31].1);
    let r2s = format!("^{}+(?<pn>{}(?&pn)?{})$", r42, r42, r31);
    let mut r2 = pcre::Pcre::compile(&r2s).unwrap();

    let p2: Vec<_> = lines
        .iter()
        .map(|s| r2.exec(&s))
        .filter(Option::is_some)
        .collect();
    println!("p2={}", p2.len());
}
