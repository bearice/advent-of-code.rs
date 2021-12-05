use std::collections::HashMap;

use advent_of_code::common::read_lines;
use regex::Regex;
#[macro_use]
extern crate lazy_static;

fn parse_mask1(line: String) -> (usize, usize) {
    let s = &line[7..];
    // println!("{}", s);
    let s1 = s.replace("X", "0");
    let n1 = usize::from_str_radix(&s1, 2).unwrap();
    let s2 = s.replace("X", "1");
    let n2 = usize::from_str_radix(&s2, 2).unwrap();
    (n1, n2)
}

fn parse_mask2(line: String) -> (usize, usize) {
    let s = &line[7..];
    // println!("{}", s);
    let s1 = s.replace("X", "0");
    let n1 = usize::from_str_radix(&s1, 2).unwrap();
    let s2 = s.replace("1", "0").replace("X", "1");
    let n2 = usize::from_str_radix(&s2, 2).unwrap();
    (n1, n2)
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
}
fn parse_mem1(line: String, mask: (usize, usize)) -> (usize, usize) {
    let m = RE.captures(&line).unwrap();
    // println!("{} {}", &m[1], &m[2]);
    let addr = m[1].parse().unwrap();
    let mut value = m[2].parse().unwrap();
    value |= mask.0;
    value &= mask.1;
    (addr, value)
}

fn parse_mem2(line: String, mask: (usize, usize)) -> (Vec<usize>, usize) {
    let m = RE.captures(&line).unwrap();
    // println!("{} {}", &m[1], &m[2]);
    let addr = m[1].parse().unwrap();
    let value = m[2].parse().unwrap();
    let addrs = mask_addr(addr, mask.1, mask.0);
    (addrs, value)
}

fn mask_addr(addr: usize, mask: usize, mask2: usize) -> Vec<usize> {
    // println!("mask={:b} mask2={:b}", mask, mask2);
    if mask == 0 {
        // println!("addr={:b}", addr | mask2);
        vec![addr | mask2]
    } else {
        let tmask = 1 << mask.trailing_zeros();
        // println!("tmask={:b}", !tmask);
        let mask = mask & !tmask;
        let mut ret = mask_addr(addr, mask, mask2);
        let mut other = ret.clone();
        for i in 0..ret.len() {
            ret[i] &= !tmask;
            other[i] |= tmask;
        }
        ret.append(&mut other);
        ret
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn mask_addr() {
        let mask = super::parse_mask2("mask = 000000000000000000000000000000X1001X".to_owned());
        let mut ret = super::mask_addr(0b101010, mask.1, mask.0);
        let mut tgt = vec![0b011010, 0b011011, 0b111010, 0b111011];
        tgt.sort();
        ret.sort();
        assert_eq!(ret, tgt);
    }

    #[test]
    fn parse_mask1() {
        assert_eq!(
            super::parse_mask1("mask = 1100X10X01001X111001X00010X00100X011".to_owned()),
            (
                0b110001000100101110010000100001000011,
                0b110011010100111110011000101001001011
            )
        );
    }
    #[test]
    fn parse_mask2() {
        assert_eq!(
            super::parse_mask2("mask = 000000000000000000000000000000X1001X".to_owned()),
            (
                0b000000000000000000000000000000010010,
                0b000000000000000000000000000000100001
            )
        );
    }
    #[test]
    fn parse_mem1() {
        let mask = super::parse_mask1("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_owned());
        assert_eq!(super::parse_mem1("mem[8] = 11".to_owned(), mask), (8, 73));
        assert_eq!(super::parse_mem1("mem[1] = 101".to_owned(), mask), (1, 101));
        assert_eq!(super::parse_mem1("mem[0] = 0".to_owned(), mask), (0, 64));
    }
    #[test]
    fn parse_mem2() {
        let mask = super::parse_mask2("mask = 000000000000000000000000000000X1001X".to_owned());
        let mut ret = super::parse_mem2("mem[42] = 11".to_owned(), mask);
        ret.0.sort();
        assert_eq!(ret, (vec![26, 27, 58, 59], 11));
    }
}

fn main() {
    let lines: Vec<String> = read_lines("./input14.txt").collect();
    part1(lines.clone());
    part2(lines.clone());
}

fn part1(lines: Vec<String>) {
    let mut mask = (0, 0);
    let mut mem = HashMap::new();
    for line in lines {
        if line.starts_with("mask") {
            mask = parse_mask1(line);
        } else {
            let (addr, value) = parse_mem1(line, mask);
            mem.insert(addr, value);
        }
    }
    let sum: usize = mem.values().sum();
    println!("p1={}", sum);
}

fn part2(lines: Vec<String>) {
    let mut mask = (0, 0);
    let mut mem = HashMap::new();
    for line in lines {
        if line.starts_with("mask") {
            mask = parse_mask2(line);
        } else {
            let (addrs, value) = parse_mem2(line, mask);
            for addr in addrs {
                mem.insert(addr, value);
            }
        }
    }
    let sum: usize = mem.values().sum();
    println!("p2={}", sum);
}
