use std::ops::Shl;

use advent_of_code::common::read_lines;
use itertools::Itertools;

fn main() {
    let input = read_lines("input16.txt").next().unwrap();
    let bits = input.chars().flat_map(char_to_bits).collect_vec();
    let root = parse_packet(&bits);

    println!("{}", ver_sum(&root));
    println!("{}", eval(&root));
}

fn ver_sum(p: &Packet) -> usize {
    p.v as usize
        + p.sub
            .as_ref()
            .map(|v| v.iter().map(ver_sum).sum::<usize>())
            .unwrap_or(0)
}

fn eval(p: &Packet) -> usize {
    match p.t {
        0 => p
            .sub
            .as_ref()
            .map(|v| v.iter().map(eval).sum::<usize>())
            .unwrap_or(p.val),
        1 => p
            .sub
            .as_ref()
            .map(|v| v.iter().map(eval).product::<usize>())
            .unwrap_or(p.val),
        2 => p
            .sub
            .as_ref()
            .map(|v| v.iter().map(eval).min().unwrap())
            .unwrap_or(p.val),
        3 => p
            .sub
            .as_ref()
            .map(|v| v.iter().map(eval).max().unwrap())
            .unwrap_or(p.val),
        4 => p.val,
        5 => {
            let a = &p.sub.as_ref().unwrap()[0];
            let b = &p.sub.as_ref().unwrap()[1];
            if eval(a) > eval(b) {
                1
            } else {
                0
            }
        }
        6 => {
            let a = &p.sub.as_ref().unwrap()[0];
            let b = &p.sub.as_ref().unwrap()[1];
            if eval(a) < eval(b) {
                1
            } else {
                0
            }
        }
        7 => {
            let a = &p.sub.as_ref().unwrap()[0];
            let b = &p.sub.as_ref().unwrap()[1];
            if eval(a) == eval(b) {
                1
            } else {
                0
            }
        }
        _ => panic!("not possible"),
    }
}
#[derive(Debug)]
struct Packet {
    v: u8,
    t: u8,
    val: usize,
    sub: Option<Vec<Packet>>,
    siz: usize,
}

fn read_buf(bits: &[u8], mut cnt: usize) -> (Vec<Packet>, usize) {
    let mut i = 0;
    let mut ret = Vec::new();
    while i + 6 < bits.len() && cnt > 0 {
        let p = parse_packet(&bits[i..]);
        i += p.siz;
        cnt -= 1;
        ret.push(p);
    }
    (ret, i)
}

fn parse_packet(bits: &[u8]) -> Packet {
    let v = bits[0..3].iter().fold(0, |acc, &b| acc.shl(1) + b);
    let t = bits[3..6].iter().fold(0, |acc, &b| acc.shl(1) + b);

    let (val, sub, siz) = if t == 4 {
        let (val, siz) = read_number(&bits[6..]);
        (val, None, siz + 6)
    } else if bits[6] == 0 {
        let val = bits[7..22]
            .iter()
            .fold(0, |acc, &b| acc.shl(1) + b as usize);
        let buf = &bits[22..22 + val];
        let (sub, _) = read_buf(buf, usize::MAX);
        (val, Some(sub), 22 + val)
    } else {
        let val = bits[7..18]
            .iter()
            .fold(0, |acc, &b| acc.shl(1) + b as usize);
        let (sub, len) = read_buf(&bits[18..], val);
        (val, Some(sub), 6 + 12 + len)
    };

    Packet {
        v,
        t,
        val,
        sub,
        siz,
    }
}

fn read_number(bits: &[u8]) -> (usize, usize) {
    let mut ret = 0;
    let mut i = 0;
    while i < bits.len() {
        ret = bits[i + 1..i + 5]
            .iter()
            .fold(ret, |acc, &b| acc.shl(1) + b as usize);
        if bits[i] == 0 {
            break;
        }
        i += 5;
    }
    (ret, i + 5)
}

fn char_to_bits(c: char) -> Vec<u8> {
    match c {
        '0' => vec![0, 0, 0, 0],
        '1' => vec![0, 0, 0, 1],
        '2' => vec![0, 0, 1, 0],
        '3' => vec![0, 0, 1, 1],
        '4' => vec![0, 1, 0, 0],
        '5' => vec![0, 1, 0, 1],
        '6' => vec![0, 1, 1, 0],
        '7' => vec![0, 1, 1, 1],
        '8' => vec![1, 0, 0, 0],
        '9' => vec![1, 0, 0, 1],
        'A' => vec![1, 0, 1, 0],
        'B' => vec![1, 0, 1, 1],
        'C' => vec![1, 1, 0, 0],
        'D' => vec![1, 1, 0, 1],
        'E' => vec![1, 1, 1, 0],
        'F' => vec![1, 1, 1, 1],
        _ => panic!("Invalid character"),
    }
}
