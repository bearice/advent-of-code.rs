use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
    ops::Shl,
};

trait Grid {
    fn get(&self, x: usize, y: usize) -> bool;
    fn get_n(&self, id: usize) -> u32;
    fn set(&mut self, x: usize, y: usize, v: bool);
    fn adj(&self, x: usize, y: usize) -> u32;
    fn parse(input: &str) -> Self;
    fn next_minute(&self) -> u32;
    fn print(&self);
}

impl Grid for u32 {
    fn get(&self, x: usize, y: usize) -> bool {
        self >> (x + y * 5) & 1 == 1
    }

    fn get_n(&self, id: usize) -> u32 {
        self >> (id - 1) & 1
    }

    fn set(&mut self, x: usize, y: usize, v: bool) {
        let mask = 1 << (x + y * 5);
        if v {
            *self |= mask;
        } else {
            *self &= !mask;
        }
    }

    fn adj(&self, x: usize, y: usize) -> u32 {
        let mut ret = 0;
        if x > 0 && self.get(x - 1, y) {
            ret += 1;
        }
        if x < 4 && self.get(x + 1, y) {
            ret += 1;
        }
        if y > 0 && self.get(x, y - 1) {
            ret += 1;
        }
        if y < 4 && self.get(x, y + 1) {
            ret += 1;
        }
        ret
    }

    fn next_minute(&self) -> u32 {
        let mut ret = *self;
        for x in 0..5 {
            for y in 0..5 {
                if self.get(x, y) {
                    ret.set(x, y, self.adj(x, y) == 1);
                } else {
                    ret.set(x, y, self.adj(x, y) == 1 || self.adj(x, y) == 2);
                }
            }
        }
        ret
    }

    fn parse(input: &str) -> Self {
        input
            .chars()
            .filter_map(|c| match c {
                '#' => Some(1u32),
                '.' => Some(0),
                _ => None,
            })
            .rev()
            .reduce(|acc, x| acc.shl(1) + x)
            .unwrap()
    }

    fn print(&self) {
        for y in 0..5 {
            for x in 0..5 {
                print!("{}", self.get(x, y) as u8);
            }
            println!();
        }
    }
}

#[derive(Clone, Debug)]
struct RecursiveGrid {
    offset: i32,
    grids: VecDeque<u32>,
}
impl RecursiveGrid {
    fn new(mut grid0: u32) -> Self {
        grid0.set(2, 2, false);
        let mut grids = VecDeque::new();
        grids.push_back(grid0);
        RecursiveGrid { offset: 0, grids }
    }

    fn get(&self, level: i32, x: usize, y: usize) -> bool {
        self.get_level(level).get(x, y)
    }

    fn set(&mut self, level: i32, x: usize, y: usize, v: bool) {
        self.get_level_mut(level).set(x, y, v)
    }

    fn adj(&self, level: i32, x: usize, y: usize) -> u32 {
        let mut ret = self.get_level(level).adj(x, y);
        match x + y * 5 + 1 {
            1 => {
                let lv = self.get_level(level - 1);
                ret += lv.get_n(8) as u32;
                ret += lv.get_n(12) as u32;
            }
            2 | 3 | 4 => {
                let lv = self.get_level(level - 1);
                ret += lv.get_n(8) as u32;
            }
            5 => {
                let lv = self.get_level(level - 1);
                ret += lv.get_n(8) as u32;
                ret += lv.get_n(14) as u32;
            }
            6 | 11 | 16 => {
                let lv = self.get_level(level - 1);
                ret += lv.get_n(12) as u32;
            }
            21 => {
                let lv = self.get_level(level - 1);
                ret += lv.get_n(12) as u32;
                ret += lv.get_n(18) as u32;
            }
            22 | 23 | 24 => {
                let lv = self.get_level(level - 1);
                ret += lv.get_n(18) as u32;
            }
            25 => {
                let lv = self.get_level(level - 1);
                ret += lv.get_n(14) as u32;
                ret += lv.get_n(18) as u32;
            }
            10 | 15 | 20 => {
                let lv = self.get_level(level - 1);
                ret += lv.get_n(14) as u32;
            }
            8 => {
                let lv = self.get_level(level + 1);
                ret += (0..5).map(|x| lv.get(x, 0) as u32).sum::<u32>();
            }
            12 => {
                let lv = self.get_level(level + 1);
                ret += (0..5).map(|y| lv.get(0, y) as u32).sum::<u32>();
            }
            18 => {
                let lv = self.get_level(level + 1);
                ret += (0..5).map(|x| lv.get(x, 4) as u32).sum::<u32>();
            }
            14 => {
                let lv = self.get_level(level + 1);
                ret += (0..5).map(|y| lv.get(4, y) as u32).sum::<u32>();
            }
            _ => {}
        }
        ret
    }

    fn next_minute(&mut self) {
        let mut ret = self.clone();
        for level in self.min_level() - 1..self.max_level() + 2 {
            for x in 0..5 {
                for y in 0..5 {
                    if x == 2 && y == 2 {
                        continue;
                    }
                    let adj = self.adj(level, x, y);
                    if self.get(level, x, y) {
                        ret.set(level, x, y, adj == 1);
                    } else {
                        ret.set(level, x, y, adj == 1 || adj == 2);
                    }
                }
            }
        }
        ret.trim();
        *self = ret;
    }

    fn trim(&mut self) {
        while self.grids.len() > 1 && self.grids[0] == 0 {
            self.grids.pop_front();
            self.offset -= 1;
        }
        while self.grids.len() > 1 && self.grids[self.grids.len() - 1] == 0 {
            self.grids.pop_back();
        }
    }

    fn get_level(&self, level: i32) -> u32 {
        if level < 0 || level >= self.grids.len() as i32 {
            0
        } else {
            self.grids[level as usize]
        }
    }

    fn get_level_mut(&mut self, level: i32) -> &mut u32 {
        let mut level = level + self.offset;
        while level < 0 || level >= self.grids.len() as i32 {
            if level < 0 {
                level += 1;
                self.offset += 1;
                self.grids.push_front(0);
            } else if level >= self.grids.len() as i32 {
                self.grids.push_back(0);
            }
        }
        &mut self.grids[level as usize]
    }

    fn max_level(&self) -> i32 {
        self.offset + self.grids.len() as i32 - 1
    }

    fn min_level(&self) -> i32 {
        -self.offset
    }

    fn count(&self) -> u32 {
        self.grids.iter().map(|g| g.count_ones()).sum()
    }
}

fn main() {
    let input = read_to_string("input24.txt").unwrap();
    let grid: u32 = Grid::parse(&input);
    let mut next = grid;
    let mut seen = HashSet::new();
    seen.insert(next);
    loop {
        next = next.next_minute();
        // println!("==={}===", seen.len());
        // next.print();
        if seen.contains(&next) {
            break;
        } else {
            seen.insert(next);
        }
    }
    println!("{}", next);

    let mut grid = RecursiveGrid::new(grid);
    for _ in 0..200 {
        grid.next_minute();
    }
    println!("{}", grid.count());
}
