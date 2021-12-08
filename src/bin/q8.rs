use std::collections::HashSet;

use advent_of_code::common::read_lines;

fn main() {
    let input = read_lines("input8.txt").collect::<Vec<_>>();
    let count = input
        .iter()
        .map(|s| {
            s.split_once('|')
                .unwrap()
                .1
                .split_whitespace()
                .filter(|w| w.len() == 2 || w.len() == 3 || w.len() == 4 || w.len() == 7)
                .count()
        })
        .sum::<usize>();
    println!("{}", count);
    let sum = input.iter().map(|s| decode(s.as_str())).sum::<u32>();
    println!("{}", sum);
}

fn decode(input: &str) -> u32 {
    let (sample, output) = input.split_once('|').unwrap();
    let samples = sample.split_whitespace().collect::<Vec<_>>();
    let mut wires = [""; 10];
    for s in samples.iter() {
        match s.len() {
            2 => wires[1] = s,
            3 => wires[7] = s,
            4 => wires[4] = s,
            7 => wires[8] = s,
            _ => (),
        }
    }
    // let _A = diff(wires[7].chars(), wires[1].chars());
    let bd = diff(wires[4].chars(), wires[1].chars());
    let abcdf = union(wires[4].chars(), wires[7].chars());
    wires[9] = find(samples.clone(), &abcdf, 6);
    let g = diff(wires[9].chars(), abcdf.into_iter());
    let e = diff(wires[8].chars(), wires[9].chars());
    let cfeg = union(vec![g[0], e[0]], wires[1].chars());
    wires[0] = find(samples.iter().cloned().filter(|s| *s != wires[9]), &cfeg, 6);
    let cf = wires[1].chars().collect::<Vec<_>>();
    wires[3] = find(samples.clone(), &cf, 5);
    let bed = union(bd, e.clone());
    wires[6] = find(samples.clone(), &bed, 6);
    let c = diff(wires[8].chars(), wires[6].chars());
    let ce = vec![c[0], e[0]];
    wires[2] = find(samples.clone(), &ce, 5);
    let bdf = diff(wires[4].chars(), c);
    wires[5] = find(samples.clone(), &bdf, 5);
    // println!("{:?}", wires);
    let outputs = output.split_whitespace().collect::<Vec<_>>();
    let mut result = 0;
    for (i, o) in outputs.iter().enumerate() {
        for j in 0..10 {
            let t = wires[j as usize];
            if o.len() == t.len() && o.chars().all(|c| t.contains(c)) {
                // println!("{}={}", o, j);
                result += j * 10_u32.pow((outputs.len() - i - 1) as u32);
            }
        }
    }
    result
}

fn find<'a>(
    samples: impl IntoIterator<Item = &'a str>,
    contains: &[char],
    length: usize,
) -> &'a str {
    samples
        .into_iter()
        .find(|s| s.len() == length && contains_all(s, contains))
        .unwrap()
}

fn diff(a: impl IntoIterator<Item = char>, b: impl IntoIterator<Item = char>) -> Vec<char> {
    let b: HashSet<_> = b.into_iter().collect();
    a.into_iter().filter(|c| !b.contains(c)).collect()
}

fn union(a: impl IntoIterator<Item = char>, b: impl IntoIterator<Item = char>) -> Vec<char> {
    let mut set = HashSet::new();
    for c in a {
        set.insert(c);
    }
    for c in b {
        set.insert(c);
    }
    set.into_iter().collect()
}

fn contains_all(s: &str, chars: &[char]) -> bool {
    chars.iter().all(|c| s.contains(*c))
}
