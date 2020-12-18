use advent_of_code::common::read_lines;

fn main() {
    let mut cubes = init();
    println!("{:?}", cubes);
    for i in 0..6 {
        cubes = cycle(cubes);
        println!("c={} {}", i, cubes.len());
    }
}
fn init() -> Vec<CubePos> {
    let mut ret = vec![];
    let lines = read_lines("./input17.txt").unwrap();
    let mut y = 0;
    let mut x = 0;
    for line in lines {
        for ch in line.unwrap().chars() {
            if ch == '#' {
                ret.push((x, y, 0, 0));
            }
            x += 1
        }
        x = 0;
        y += 1;
    }
    ret
}
use std::collections::HashMap;
fn cycle(cubes: Vec<CubePos>) -> Vec<CubePos> {
    let neighbors: Vec<CubePos> = cubes.iter().flat_map(|c| neighbors(c)).collect();
    let mut cube_map = HashMap::new();
    for c in cubes.into_iter() {
        cube_map.insert(c, (true, 0));
    }
    for p in neighbors {
        if let Some(c) = cube_map.get_mut(&p) {
            c.1 += 1;
        } else {
            cube_map.insert(p, (false, 1));
        }
    }
    let mut ret = vec![];

    for (k, mut c) in cube_map.into_iter() {
        if c.0 {
            c.0 = c.1 == 2 || c.1 == 3;
        } else {
            c.0 = c.1 == 3;
        }
        // println!("{:?} {:?}", k, c);
        if c.0 {
            ret.push(k);
        }
    }
    // ret.sort_by_key(|x| (x.1, x.0));
    // print_cube(ret.iter());
    ret
}

// fn print_cube<'a, I>(v: I)
// where
//     I: Iterator<Item = &'a CubePos>,
// {
//     let t: Vec<CubePos> = v.filter(|x| x.2 == 0).map(|x| *x).collect();
//     println!("{:?}", t);
//     // let min_x = t.iter().map(|x| x.pos.0).min().unwrap();
//     // let min_y = t.iter().map(|x| x.pos.1).min().unwrap();
// }
type CubePos = (i32, i32, i32, i32);
fn neighbors(p: &CubePos) -> Vec<CubePos> {
    let mut ret = vec![];
    for x in p.0 - 1..p.0 + 2 {
        for y in p.1 - 1..p.1 + 2 {
            for z in p.2 - 1..p.2 + 2 {
                for w in p.3 - 1..p.3 + 2 {
                    if (x, y, z, w) != *p {
                        ret.push((x, y, z, w));
                    }
                }
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    #[test]
    fn neighbors() {
        let n = super::neighbors(&(0, 0, 0, 0));
        assert!(n.len() == 80);
        println!("{:?}", n)
    }
}
