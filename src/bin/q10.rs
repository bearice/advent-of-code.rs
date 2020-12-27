use advent_of_code::common::read_lines;
use cached::proc_macro::cached;

#[cached]
fn paths(n: usize) -> usize {
    match n {
        0 => 1,
        1 => 1,
        2 => 2,
        3 => 4,
        x => paths(x - 1) + paths(x - 2) + paths(x - 3),
    }
}

fn main() {
    let mut numbers: Vec<usize> = read_lines("./input10.txt")
        .unwrap()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();

    numbers.push(0);
    numbers.sort();
    let mut i = numbers.iter();
    i.next();
    let diff: Vec<_> = numbers.iter().zip(i).map(|x| x.1 - x.0).collect();
    println!("{:?}", diff);
    let n = diff.iter().fold((0, 1), |mut acc, x| {
        match *x {
            1 => acc.0 += 1,
            3 => acc.1 += 1,
            _ => (),
        };
        acc
    });
    println!("{:?}", n.0 * n.1);
    let mut x = Vec::new();
    let mut cnt = 0;
    for i in 0..diff.len() {
        if diff[i] == 1 {
            cnt += 1;
        } else {
            x.push(cnt);
            cnt = 0;
        }
    }

    println!("{}", x.into_iter().map(paths).fold(1, |a, x| a * x));
}
//(0), 1, 2, 3

// 1=>1
// 2=>2
// 3=>4
// 4=>7
// 5=>13
