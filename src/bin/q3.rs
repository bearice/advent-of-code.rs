use std::ops::{Not, Shl, Shr};

use advent_of_code::common::read_lines;

fn main() {
    let data = read_lines("./input3.txt").collect::<Vec<_>>();
    let data = data.iter().map(|x| x.as_bytes()).collect::<Vec<_>>();
    let width = data[0].len();
    let ans = count_bits(&data);
    let gamma = bin_to_num(&ans);
    let epsilon = gamma.not().shl(32 - width).shr(32 - width);
    println!("gamma={} epsilon={}", gamma, epsilon);
    let ans = gamma * epsilon;
    println!("a1={}", ans);

    let o2 = find_value(&data, 0, false);
    let co2 = find_value(&data, 0, true);
    let ans = o2 * co2;
    println!("o2={} co2={} ans={}", o2, co2, ans);
}

fn bin_to_num(data: &[u8]) -> u32 {
    let mut ans = 0;
    for i in 0..data.len() {
        ans += u32::from(data[i]) * 2u32.pow((data.len() - i - 1) as u32);
    }
    ans
}

fn count_bits(data: &[&[u8]]) -> Vec<u8> {
    let width = data[0].len();
    let ret = data.iter().fold(vec![(0, 0); width], |mut acc, line| {
        for i in 0..width {
            let a = acc[i];
            acc[i] = if line[i] == 48 {
                (a.0 + 1, a.1)
            } else {
                (a.0, a.1 + 1)
            }
        }
        acc
    });
    ret.into_iter()
        .map(|x| if x.0 > x.1 { 0 } else { 1 })
        .collect()
}

fn find_value(data: &[&[u8]], idx: usize, neg: bool) -> u32 {
    if data.len() == 1 {
        let data = data[0].iter().map(|x| x - 48).collect::<Vec<_>>();
        return bin_to_num(&data);
    }
    let flag = count_bits(data)[idx] + 48;
    let data = data
        .iter()
        .cloned()
        .filter(|x| (x[idx] == flag) ^ neg)
        .collect::<Vec<_>>();
    find_value(&data, idx + 1, neg)
}
