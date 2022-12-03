use std::collections::HashSet;

use advent_of_code::common::read_lines;
use itertools::Itertools;

fn main() {
    let lines = read_lines("./input3.txt").collect_vec();
    print_total(q1(&lines));
    print_total(q2(&lines));
}

fn q1(lines: &[String]) -> Vec<HashSet<char>> {
    lines
        .iter()
        .map(|x| split_in_half(x))
        .map(|x| find_common_char([x.0, x.1]))
        .collect_vec()
}

fn q2(lines: &[String]) -> Vec<HashSet<char>> {
    lines
        .iter()
        .map(String::as_str)
        .chunks(3)
        .into_iter()
        .map(find_common_char)
        .collect_vec()
}

fn print_total(commons: Vec<HashSet<char>>) {
    let priorities = commons
        .into_iter()
        .map(|x| x.into_iter().map(priority).sum::<i32>())
        .collect_vec();
    println!("{}", priorities.iter().sum::<i32>());
}

// "abcd" -> ("ab","cd")
fn split_in_half(s: &str) -> (&str, &str) {
    let mid = s.len() / 2;
    s.split_at(mid)
}

fn find_common_char<'a, T: IntoIterator<Item = &'a str>>(strings: T) -> HashSet<char> {
    let mut common = HashSet::new();

    for s in strings {
        let chars = s.chars().collect::<HashSet<char>>();

        if common.is_empty() {
            common = chars;
        } else {
            common = common.intersection(&chars).cloned().collect();
        }
    }

    common
}

//Lowercase item types a through z have priorities 1 through 26.
//Uppercase item types A through Z have priorities 27 through 52.
fn priority(ch: char) -> i32 {
    let decimal_value = ch.to_digit(36).unwrap_or(0) as i32 - 9;
    if ch.is_uppercase() {
        decimal_value + 26
    } else {
        decimal_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('Z'), 52);
    }
}
