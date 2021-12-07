use advent_of_code::common::read_lines;
fn main() {
    let pos = read_lines("./input7.txt")
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let min = pos.iter().min().unwrap();
    let max = pos.iter().max().unwrap();
    let mut least = (i32::max_value(), i32::max_value());
    for i in *min..=*max {
        let delta = pos.iter().map(|x| i32::abs(*x - i));
        let sum1 = delta.clone().sum();
        let sum2 = delta.map(|x| (1 + x) * x / 2).sum();
        least.0 = i32::min(least.0, sum1);
        least.1 = i32::min(least.1, sum2);
    }

    println!("{:?}", least);
}
