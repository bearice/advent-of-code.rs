use std::{
    collections::HashSet,
    fmt::Display,
    iter::FromIterator,
    ops::{Add, Sub},
    rc::Rc,
};

use advent_of_code::common::ReadChunks;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn rotate(&self, n: u8) -> Vec3 {
        let Vec3 { x, y, z } = *self;
        // rotate coordinate system so that x-axis points in the possible 6 directions
        let (x, y, z) = match n % 6 {
            0 => (x, y, z),
            1 => (-x, y, -z),
            2 => (y, -x, z),
            3 => (-y, x, z),
            4 => (z, y, -x),
            5 => (-z, y, x),
            _ => unreachable!(),
        };
        // rotate around x-axis:
        let (x, y, z) = match (n / 6) % 4 {
            0 => (x, y, z),
            1 => (x, -z, y),
            2 => (x, -y, -z),
            3 => (x, z, -y),
            _ => unreachable!(),
        };
        Vec3 { x, y, z }
    }
    fn length(&self) -> usize {
        (self.x * self.x) as usize + (self.y * self.y) as usize + (self.z * self.z) as usize
    }
    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{},{}]", self.x, self.y, self.z)
    }
}

#[derive(Debug, Default)]
struct Scanner {
    id: u32,
    probes: HashSet<Vec3>,
    orgin: Vec3,
    rotation: u8,
    signature: Rc<HashSet<usize>>,
}

impl Scanner {
    fn signature<B>(&self) -> B
    where
        B: FromIterator<usize>,
    {
        self.probes
            .iter()
            .tuple_combinations()
            .map(|(a, b)| {
                let c = a - b;
                c.length()
            })
            .collect()
    }

    fn rotate(&self, n: u8) -> Self {
        Self {
            id: self.id,
            probes: self.probes.iter().map(|p| p.rotate(n)).collect(),
            orgin: self.orgin.rotate(n),
            rotation: n,
            signature: self.signature.clone(),
        }
    }
    fn translate(&self, n: Vec3) -> Self {
        Self {
            id: self.id,
            probes: self.probes.iter().map(|p| p + &n).collect(),
            orgin: &self.orgin + &n,
            rotation: self.rotation,
            signature: self.signature.clone(),
        }
    }
}

impl From<Vec<String>> for Scanner {
    fn from(lines: Vec<String>) -> Self {
        let id = lines[0].split(' ').nth(2).unwrap().parse().unwrap();
        let probes = lines
            .iter()
            .skip(1)
            .map(|line| {
                let mut coords = line.split(',');
                let x = coords.next().unwrap().parse().unwrap();
                let y = coords.next().unwrap().parse().unwrap();
                let z = coords.next().unwrap().parse().unwrap();
                Vec3 { x, y, z }
            })
            .collect();
        Scanner {
            id,
            probes,
            ..Default::default()
        }
    }
}
fn main() {
    let mut scanners = ReadChunks::new("./input19.txt")
        .map(Into::into)
        .collect::<Vec<Scanner>>();

    scanners
        .iter_mut()
        .for_each(|s| s.signature = Rc::new(s.signature()));

    let mut known = vec![scanners.remove(0)];
    let mut skip = HashSet::new();
    while !scanners.is_empty() {
        let mut found = None;
        let mut idx = 0;
        'outer: for root in &known {
            for (i, test) in scanners.iter().enumerate() {
                if skip.contains(&(root.id, test.id)) {
                    continue;
                }
                // println!("{} {}", root.id, test.id);

                found = find_match(root, test);
                // println!("{:?}", found);
                if found.is_some() {
                    println!("found: {} => {}", root.id, test.id);
                    idx = i;
                    break 'outer;
                } else {
                    skip.insert((root.id, test.id));
                }
            }
        }
        if found.is_none() {
            panic!("no match found");
        }
        known.push(found.unwrap());
        scanners.remove(idx);
    }
    let all_probes = known.iter().fold(HashSet::new(), |set, s| {
        set.union(&s.probes).cloned().collect()
    });
    // println!("{:?}", all_probes.iter().sorted().collect::<Vec<_>>());
    println!("{:?}", all_probes.len());

    let max_distance = known
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (&a.orgin - &b.orgin).manhattan_distance())
        .max()
        .unwrap();

    println!("{}", max_distance);
}

fn find_match(root: &Scanner, check: &Scanner) -> Option<Scanner> {
    if root.signature.intersection(&check.signature).count() < 66 {
        return None;
    }
    for rotate in 0..24 {
        let rotated = check.rotate(rotate);
        for p1 in &root.probes {
            for p2 in &rotated.probes {
                let delta = p1 - p2;
                let translated = rotated.translate(delta);
                let count = root.probes.intersection(&translated.probes).count();
                // println!("{} {}", rotate, count);
                if count >= 12 {
                    return Some(translated);
                }
            }
        }
    }
    None
}
