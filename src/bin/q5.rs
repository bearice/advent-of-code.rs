use advent_of_code::common::read_lines;
fn main() {
    let lines = read_lines("./input5.txt").unwrap();
    let mut numbers: Vec<isize> = lines
        .map(|x| {
            x.unwrap()
                .replace("F", "0")
                .replace("L", "0")
                .replace("B", "1")
                .replace("R", "1")
        })
        .map(|x| isize::from_str_radix(&x, 2).unwrap())
        .collect();
    numbers.sort();
    println!("max: {}", numbers.last().unwrap());
    for i in 0..numbers.len() - 1 {
        if numbers[i] + 1 < numbers[i + 1] {
            println!("missing: {}", numbers[i] + 1)
        }
    }
}
