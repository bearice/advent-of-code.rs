use advent_of_code::common::read_lines;
use regex::Regex;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RULES: HashMap<String, Regex> = {
        let mut m = HashMap::<String, Regex>::new();
        m.insert(
            "byr".to_owned(),
            Regex::new(r"^((19[2-9][0-9])|(200[0-2]))$").unwrap(),
        );
        m.insert("iyr".to_owned(), Regex::new(r"^(201[0-9])|2020$").unwrap());
        m.insert("eyr".to_owned(), Regex::new(r"^(202[0-9])|2030$").unwrap());
        m.insert(
            "hgt".to_owned(),
            Regex::new(r"^((1([5-8][0-9])cm|1(9[0-3])cm)|((59|6[0-9]|7[0-6])in))$").unwrap(),
        );
        m.insert("hcl".to_owned(), Regex::new(r"^(#[0-9a-f]{6})$").unwrap());
        m.insert(
            "ecl".to_owned(),
            Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap(),
        );
        m.insert("pid".to_owned(), Regex::new(r"^([0-9]{9})$").unwrap());
        // m.insert("cid".to_owned(), Regex::new(r".*").unwrap());
        m
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn hgt() {
        let r = super::RULES.get("hgt").unwrap();
        for i in 1..200 {
            let s = format!("{}cm", &i);
            if (150..194).contains(&i) {
                assert!(r.is_match(&s), s);
            } else {
                assert!(!r.is_match(&s), s);
            }
        }

        for i in 1..200 {
            let s = format!("{}in", &i);
            if (59..77).contains(&i) {
                assert!(r.is_match(&s), s);
            } else {
                assert!(!r.is_match(&s), s);
            }
        }

        assert!(!r.is_match("199cm"));
        assert!(!r.is_match("158in"));
        assert!(!r.is_match("sfe1cm"));
        assert!(!r.is_match("aa160cmbb"));
    }
}

fn validate(person: HashMap<String, String>) -> bool {
    if person.len() != 7 {
        // println!("len={} person={:?}", person.len(), person);
        return false;
    }
    for k in RULES.keys() {
        let r = RULES.get(k).unwrap();
        if let Some(v) = person.get(k) {
            if !r.is_match(v) {
                // println!("k={} v={}", k, v);
                return false;
            }
        } else {
            println!("k={}", k);
            return false;
        }
    }
    println!("{:?}", person);
    true
}

fn main() {
    let lines = read_lines("./input4.txt").unwrap();
    let mut cur = HashMap::new();
    let mut cnt = 0;
    for line in lines {
        let line = line.unwrap();
        if line.len() > 0 {
            for pair in line.split_ascii_whitespace() {
                let mut i = pair.split(':');
                let k = i.next().unwrap();
                let v = i.next().unwrap();
                cur.insert(k.to_owned(), v.to_owned());
            }
        } else {
            cur.remove("cid");
            if validate(cur) {
                cnt += 1;
            }
            cur = HashMap::new();
        }
    }
    println!("{}", cnt)
}
