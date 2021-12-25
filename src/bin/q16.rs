use advent_of_code::common::read_lines;
use itertools::Itertools;

fn pattern_for(n: usize) -> impl Iterator<Item = i8> {
    struct Iter {
        repeats: usize,
        idx: usize,
    }
    impl Iterator for Iter {
        type Item = i8;
        fn next(&mut self) -> Option<Self::Item> {
            const PATTERN: [i8; 4] = [0, 1, 0, -1];
            self.idx += 1;
            Some(PATTERN[self.idx / self.repeats % PATTERN.len()])
        }
    }
    Iter {
        repeats: n + 1,
        idx: 0,
    }
}

fn fft(input: &[i8]) -> Vec<i8> {
    (0..input.len())
        .map(|i| {
            (input
                .iter()
                .zip(pattern_for(i))
                .map(|(&a, b)| a as i32 * b as i32)
                .sum::<i32>()
                .abs()
                % 10) as i8
        })
        .collect()
}

fn fft_tail(input: &[i8]) -> Vec<i8> {
    let mut output = vec![0; input.len()];
    let mut sum = 0;
    for i in 0..input.len() {
        let i = input.len() - 1 - i;
        let n = input[i] as i32;
        sum += n;
        output[i] = (sum % 10) as i8;
    }
    output
}

fn main() {
    let line = read_lines("input16.txt")
        .next()
        .unwrap()
        .chars()
        .map(|x| x as i8 - 48)
        .collect_vec();

    {
        let mut line = line.clone();
        for _ in 0..100 {
            line = fft(&line);
        }
        println!("{:?}", &line[0..8].iter().map(ToString::to_string).join(""));
    }
    {
        let offset = line[0..7].iter().fold(0, |acc, &x| acc * 10 + x as usize);
        let mut tail = line.repeat(10000)[offset..].to_vec();
        for _ in 0..100 {
            tail = fft_tail(&tail);
        }
        println!("{:?}", &tail[0..8].iter().map(ToString::to_string).join(""));
    }
}
