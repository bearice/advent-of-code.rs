use advent_of_code::common::read_lines;

fn main() {
    let ops = read_lines("input10.txt").map(parse);
    let mut reg = 1;
    let mut cycles = vec![];
    for op in ops {
        match op {
            None => {
                cycles.push(reg);
            }
            Some(x) => {
                cycles.push(reg);
                cycles.push(reg);
                reg += x;
            }
        }
    }
    let mut totals = 0;
    for i in [20, 60, 100, 140, 180, 220] {
        totals += i as i32 * cycles[i - 1];
    }
    println!("{}", totals);
    for y in 0..6 {
        let mut row = String::new();
        for x in 0..40 {
            let i = x + y * 40;
            let sprite = cycles[i as usize];
            if (sprite - 1..=sprite + 1).contains(&x) {
                row.push('#');
            } else {
                row.push(' ');
            }
        }
        println!("{}", row);
    }
}

fn parse(str: String) -> Option<i32> {
    match &str[0..4] {
        "noop" => None,
        "addx" => Some(str[5..].parse().unwrap()),
        _ => panic!("Invalid input"),
    }
}
