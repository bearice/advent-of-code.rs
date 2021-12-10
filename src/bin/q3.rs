use std::collections::BTreeSet;

use advent_of_code::common::read_lines;

fn main() {
    let input = read_lines("input3.txt");
    let lines = input.map(|line| {
        line.split(',')
            .map(|s| (s.chars().next().unwrap(), s[1..].parse::<i32>().unwrap()))
            .collect::<Vec<_>>()
    });
    let points = lines
        .map(|line| line.into_iter().fold(vec![], points))
        .collect::<Vec<_>>();
    let crossings = points
        .clone()
        .into_iter()
        .map(|points| points.into_iter().collect::<BTreeSet<_>>())
        .reduce(|p1, p2| p1.intersection(&p2).cloned().collect::<BTreeSet<_>>())
        .unwrap();
    let closest = crossings
        .into_iter()
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap();
    println!("{}", closest);

    for (i, p1) in points[0].iter().enumerate() {
        for (j, p2) in points[1].iter().enumerate() {
            if p1 == p2 {
                println!("{},{},{}", i, j, i + j + 2);
                return;
            }
        }
    }
}

fn points(mut points: Vec<(i32, i32)>, cmd: (char, i32)) -> Vec<(i32, i32)> {
    let from = points.last().unwrap_or(&(0, 0));
    let mut x = from.0;
    let mut y = from.1;
    let mut dx = 0;
    let mut dy = 0;
    match cmd.0 {
        'U' => dy = 1,
        'D' => dy = -1,
        'L' => dx = -1,
        'R' => dx = 1,
        _ => panic!("invalid direction"),
    }
    for _ in 0..cmd.1 {
        x += dx;
        y += dy;
        points.push((x, y));
    }
    points
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_points() {
        let input = ('U', 2);
        let expected = vec![(0, 1), (0, 2)];
        assert_eq!(super::points(vec![], input), expected);
    }
}
