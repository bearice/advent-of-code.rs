use advent_of_code::common::read_lines;

fn main() {
    let (start, end): (u32, u32) = read_lines("./input4.txt")
        .next()
        .unwrap()
        .split_once('-')
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .unwrap();
    let range = start..=end;

    let pass = range.filter(check1).collect::<Vec<_>>();
    println!("{}", pass.len());
    let pass = pass.into_iter().filter(check2).collect::<Vec<_>>();
    println!("{:?}", pass);
    println!("{}", pass.len());
}

fn check1(x: &u32) -> bool {
    let x = x.to_string();
    let chars = x.chars();
    let mut two_chars = chars.zip(x.chars().skip(1));
    two_chars.clone().any(|(a, b)| a == b) && two_chars.all(|(a, b)| a <= b)
}

fn check2(x: &u32) -> bool {
    let x = x.to_string();
    for ch in x.chars() {
        if x.matches(ch).count() == 2 {
            // println!("x={} ch={}", x, ch);
            return true;
        }
    }
    false
}
