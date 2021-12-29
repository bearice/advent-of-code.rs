use advent_of_code::common::read_lines;
use itertools::Itertools;

fn deal_stack(total: i64, pos: i64) -> i64 {
    total - pos - 1
}

fn deal_cut(total: i64, cut: i64, pos: i64) -> i64 {
    let cut = if cut < 0 { total + cut } else { cut } % total;
    if pos < cut {
        total - cut + pos
    } else {
        pos - cut
    }
}

fn deal_inc(total: i64, inc: i64, pos: i64) -> i64 {
    pos * inc % total
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let input = read_lines("input22.txt").collect_vec();
    println!("{}", solve1(&input, 2019, 10007));
    println!("{}", solve2(&input));
}

fn solve1(input: &[String], i: i64, total: i64) -> i64 {
    let mut i = i;
    for line in input.iter() {
        if let Some(inc) = line.strip_prefix("deal with increment ") {
            i = deal_inc(total, inc.parse().unwrap(), i);
        } else if line.starts_with("deal into new stack") {
            i = deal_stack(total, i);
        } else if let Some(cut) = line.strip_prefix("cut ") {
            i = deal_cut(total, cut.parse().unwrap(), i);
        } else {
            panic!("unknown line: {}", line);
        }
    }
    i
}

// Some magic i totally don't understand
fn solve2(input: &[String]) -> i128 {
    use mod_exp::mod_exp;

    const M: i128 = 119_315_717_514_047;
    const N: i128 = 101_741_582_076_661;

    // Convert the whole process to a linear equation: ax + b
    let (a, b) = input.iter().rev().fold((1, 0), |(a, b), cmd| {
        let (a_new, b_new) = match cmd.rsplit_once(' ').unwrap() {
            (_, "stack") => (-a, -b - 1),
            ("cut", n) => (a, b + n.parse::<i128>().unwrap()),
            ("deal with increment", n) => {
                let n = n.parse::<i128>().unwrap();
                let n = mod_exp(n, M - 2, M);
                (a * n, b * n)
            }
            _ => panic!("unknown command: {}", cmd),
        };
        (a_new % M, b_new % M)
    });

    // Applying the function n times simplifies to:
    // x * a^n + b * (a^n - 1) / (a-1)
    let term1 = 2020 * mod_exp(a, N, M) % M;
    let tmp = (mod_exp(a, N, M) - 1) * mod_exp(a - 1, M - 2, M) % M;
    let term2 = b * tmp % M;
    (term1 + term2) % M
}
