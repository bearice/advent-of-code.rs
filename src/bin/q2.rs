use advent_of_code::common::read_lines;
fn main() {
    let lines = read_lines("./input2.txt")
        .map(|s| {
            let (x, y) = s.split_once(' ').unwrap();
            (x.to_owned(), y.parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();
    let answer1 = lines
        .iter()
        .fold((0, 0), |acc, (cmd, arg)| match cmd.as_str() {
            "forward" => (acc.0 + arg, acc.1),
            "up" => (acc.0, acc.1 - arg),
            "down" => (acc.0, acc.1 + arg),
            x => panic!("Invalid command: {}", x),
        });
    println!("a1={}", answer1.0 * answer1.1);

    let answer2 = lines
        .iter()
        .fold((0, 0, 0), |acc, (cmd, arg)| match cmd.as_str() {
            "forward" => (acc.0 + arg, acc.1 + (arg * acc.2), acc.2),
            "up" => (acc.0, acc.1, acc.2 - arg),
            "down" => (acc.0, acc.1, acc.2 + arg),
            x => panic!("Invalid command: {}", x),
        });
    println!("a1={}", answer2.0 * answer2.1);
}
