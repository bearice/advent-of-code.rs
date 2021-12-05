use std::collections::HashMap;

use advent_of_code::common::read_lines;

#[derive(Debug, Clone, Copy)]
struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl Line {
    fn parse(s: String) -> Self {
        let s = s.replace(" -> ", ",");
        let mut parts = s.split(",");
        Line {
            x1: parts.next().unwrap().parse().unwrap(),
            y1: parts.next().unwrap().parse().unwrap(),
            x2: parts.next().unwrap().parse().unwrap(),
            y2: parts.next().unwrap().parse().unwrap(),
        }
    }
    fn points(&self) -> Vec<(u32, u32)> {
        let mut points = vec![];
        if self.x1 == self.x2 {
            for y in u32::min(self.y1, self.y2)..=u32::max(self.y1, self.y2) {
                points.push((self.x1, y));
            }
        } else if self.y1 == self.y2 {
            for x in u32::min(self.x1, self.x2)..=u32::max(self.x1, self.x2) {
                points.push((x, self.y1));
            }
        } else {
            let (x1, x2, y1, y2) = if self.x1 > self.x2 {
                (self.x2, self.x1, self.y2, self.y1)
            } else {
                (self.x1, self.x2, self.y1, self.y2)
            };
            for x in x1..=x2 {
                let d = x - x1;
                let y = if y1 < y2 { y1 + d } else { y1 - d };
                points.push((x, y));
            }
        }
        points
    }
}

fn main() {
    let lines = read_lines("./input5.txt")
        .map(Line::parse)
        .collect::<Vec<_>>();

    let a1 = count_overlaps(
        lines
            .iter()
            .filter(|l| l.x1 == l.x2 || l.y1 == l.y2)
            .cloned(),
    );
    let a2 = count_overlaps(lines.into_iter());
    println!("a1={}, a2={}", a1, a2);
}

fn count_overlaps(lines: impl Iterator<Item = Line>) -> usize {
    let mut points = HashMap::new();
    for line in lines {
        for point in line.points() {
            points.insert(point, points.get(&point).unwrap_or(&0) + 1);
        }
    }
    points.values().filter(|&v| *v > 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let line = Line::parse("1,1,1,3".to_string());
        assert_eq!(line.x1, 1);
    }

    #[test]
    fn test_points() {
        let line = Line::parse("1,1,1,3".to_string());
        assert_eq!(line.points(), vec![(1, 1), (1, 2), (1, 3)]);
    }
}
