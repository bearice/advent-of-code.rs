use advent_of_code::common::read_lines;
use itertools::Itertools;

fn main() {
    let input = read_lines("input15.txt").map(parse).collect_vec();
    let ranges = find_ranges(&input, 2000000, None);
    println!("{}", ranges.iter().map(|(s, e)| e - s).sum::<i32>());
    for y in 0..4000000 {
        let ranges = find_ranges(&input, y, Some((0, 4000000)));
        if ranges.len() > 1 {
            let x = (ranges[0].1 + 1) as usize;
            println!("{}", x * 4000000 + y as usize);
            break;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

//Sensor at x=2389280, y=2368338: closest beacon is at x=2127703, y=2732666
fn parse(input: String) -> (Point, Point) {
    let input = input
        .split_ascii_whitespace()
        .filter_map(|s| s.trim_matches(&['x', 'y', '=', ',', ':'][..]).parse().ok())
        .collect_vec();
    (
        Point {
            x: input[0],
            y: input[1],
        },
        Point {
            x: input[2],
            y: input[3],
        },
    )
}

fn find_ranges(input: &[(Point, Point)], row: i32, limit: Option<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut ranges = vec![];
    for (sensor, beacon) in input {
        let distance = beacon.distance(sensor);
        let dy = (sensor.y - row).abs();
        let dx = distance - dy;
        if dx >= 0 {
            let mut range = (sensor.x - dx, sensor.x + dx);
            if let Some((min, max)) = limit {
                range.0 = range.0.max(min);
                range.1 = range.1.min(max);
            }
            ranges.push(range);
        }
    }
    merge_ranges(ranges)
}

fn merge_ranges(mut ranges: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    ranges.sort();
    let mut merged = vec![ranges[0]];
    let mut i = 1;
    let mut next = 0;
    while i < ranges.len() {
        let r = ranges[i];
        // println!("last={:?}, r={:?}", merged[next], r);
        if r.0 > merged[next].1 {
            merged.push(r);
            next += 1;
        } else {
            merged[next].1 = merged[next].1.max(r.1);
        }
        i += 1;
    }
    merged
}
