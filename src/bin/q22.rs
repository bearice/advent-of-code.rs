use advent_of_code::common::read_lines;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, Default)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn overlap(&self, other: &Range) -> bool {
        self.start <= other.end && other.start <= self.end
    }
    fn intersection(&self, other: &Range) -> Option<Range> {
        if self.overlap(other) {
            Some(Range {
                start: self.start.max(other.start),
                end: self.end.min(other.end),
            })
        } else {
            None
        }
    }
    fn size(&self) -> i64 {
        self.end - self.start + 1
    }
}

impl From<(&str, &str)> for Range {
    fn from((start, end): (&str, &str)) -> Self {
        Range {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }
}

impl From<(i64, i64)> for Range {
    fn from((start, end): (i64, i64)) -> Self {
        Range { start, end }
    }
}

#[derive(Debug, Clone, Default)]
struct Box {
    x: Range,
    y: Range,
    z: Range,
    vacuums: Vec<Box>,
}
impl Box {
    fn new(x: Range, y: Range, z: Range) -> Self {
        Box {
            x,
            y,
            z,
            vacuums: Vec::new(),
        }
    }
    fn from_string(str: String) -> (bool, Self) {
        let on = str.split_once(' ').unwrap();
        let mut ranges =
            on.1.split(',')
                .map(|x| x.split_once('=').unwrap().1)
                .map(|s| s.split_once("..").unwrap().into());

        let x = ranges.next().unwrap();
        let y = ranges.next().unwrap();
        let z = ranges.next().unwrap();
        let on = on.0 == "on";
        (on, Self::new(x, y, z))
    }
    fn overlap(&self, other: &Box) -> bool {
        self.x.overlap(&other.x) && self.y.overlap(&other.y) && self.z.overlap(&other.z)
    }
    fn intersection(&self, other: &Box) -> Option<Box> {
        let x = self.x.intersection(&other.x);
        let y = self.y.intersection(&other.y);
        let z = self.z.intersection(&other.z);
        if x.is_none() || y.is_none() || z.is_none() {
            return None;
        }
        Some(Box::new(x.unwrap(), y.unwrap(), z.unwrap()))
    }
    fn size(&self) -> i64 {
        self.x.size() * self.y.size() * self.z.size()
            - self.vacuums.iter().map(Box::size).sum::<i64>()
    }
    fn remove(&mut self, other: &Box) {
        let shaved = self.intersection(other);
        if shaved.is_none() {
            return;
        }
        let shaved = shaved.unwrap();
        for vacuum in &mut self.vacuums {
            vacuum.remove(&shaved);
        }
        self.vacuums.push(shaved)
    }
}

fn main() {
    let boxes = read_lines("input22.txt")
        .map(Box::from_string)
        .collect_vec();

    let bound = Box::new((-50, 50).into(), (-50, 50).into(), (-50, 50).into());
    let boxes1 = boxes
        .iter()
        .filter(|x| x.1.overlap(&bound))
        .cloned()
        .collect_vec();
    let size1 = size_of(&boxes1);
    println!("{}", size1);
    println!("{}", size_of(&boxes));
}

fn size_of(input: &[(bool, Box)]) -> i64 {
    let mut boxes: Vec<Box> = Vec::new();
    for (on, b) in input {
        for a in boxes.iter_mut() {
            a.remove(b);
        }
        if *on {
            boxes.push(b.clone());
        }
    }
    boxes.iter().map(Box::size).sum::<i64>()
}
