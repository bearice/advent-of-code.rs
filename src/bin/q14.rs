use std::collections::HashMap;

use advent_of_code::common::ReadChunks;

fn main() {
    let mut chunks = ReadChunks::new("input14.txt");
    let template = chunks.next().unwrap();
    let pairs = chunks.next().unwrap();
    let chars = template[0].as_bytes();
    let mut template =
        chars
            .iter()
            .zip(chars.iter().skip(1))
            .fold(HashMap::new(), |mut acc, (&a, &b)| {
                *acc.entry((a, b)).or_insert(0) += 1;
                acc
            });

    // add markers of head and tail
    template.insert((0, chars[0]), 1);
    template.insert((chars[chars.len() - 1], 0), 1);

    let pairs = pairs
        .iter()
        .map(|s| {
            let s = s.as_bytes();
            ((s[0], s[1]), s[6])
        })
        .collect();

    for i in 1..=40 {
        round(&mut template, &pairs);
        if i == 10 || i == 40 {
            println!("count[{}] = {}", i, count(&template));
        }
    }
}

fn count(template: &HashMap<(u8, u8), usize>) -> usize {
    let mut count = template
        .iter()
        .fold(HashMap::new(), |mut acc, (&(c1, c2), &count)| {
            *acc.entry(c1).or_insert(0) += count;
            *acc.entry(c2).or_insert(0) += count;
            acc
        });

    // remove head and tail markers
    count.remove(&0);

    // each char was counted twice, so divide by 2
    count.iter_mut().for_each(|(_, v)| *v /= 2);

    // println!("{:?}", count);
    let max = count.iter().max_by_key(|&(_, &v)| v).unwrap();
    let min = count.iter().min_by_key(|&(_, &v)| v).unwrap();
    max.1 - min.1
}

fn round(template: &mut HashMap<(u8, u8), usize>, pairs: &HashMap<(u8, u8), u8>) {
    let mut ret = HashMap::new();
    for (&(a, b), &count) in template.iter() {
        let c = pairs.get(&(a, b));
        if let Some(&c) = c {
            *ret.entry((a, c)).or_insert(0) += count;
            *ret.entry((c, b)).or_insert(0) += count;
        } else {
            ret.insert((a, b), count);
        }
    }
    *template = ret
}
