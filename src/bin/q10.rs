use std::collections::{HashSet, LinkedList};

use advent_of_code::common::read_lines;
use itertools::Itertools;

fn main() {
    let map = read_lines("./input10.txt")
        .map(|x| {
            x.char_indices()
                .filter_map(|(x, c)| if c != '.' { Some(x) } else { None })
                .collect_vec()
        })
        .enumerate()
        .flat_map(|(y, x)| x.into_iter().map(move |x| (x, y)))
        .collect::<HashSet<_>>();

    /*
        let p = (0, 0);
        let n = count_visible(&map, p);
        println!("{:?}", n);
        print!("  ");
        for x in 0..10 {
            print!("{}", x);
        }
        println!();
        for y in 0..10 {
            print!("{} ", y);
            for x in 0..10 {
                if (x, y) == p {
                    print!("@");
                } else if n.contains(&(x, y)) {
                    print!("X");
                } else if map.contains(&(x, y)) {
                    print!("o");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    */
    let max = map
        .iter()
        .map(|&p| count_visible(&map, p).len())
        .zip(map.iter())
        .max()
        .unwrap();
    println!("{:?}", max);

    let mut angles = map
        .iter()
        .map(|&to| angle_of(*max.1, to))
        .zip(map.iter())
        .map(|((angle, distance), point)| {
            (
                (angle * 1e6).floor() as usize,
                (distance * 1e6).floor() as usize,
                point,
            )
        })
        .sorted()
        .collect::<LinkedList<_>>();

    angles.pop_front();
    let mut dead = vec![];
    while let Some(x) = angles.pop_front() {
        dead.push(x.2);
        let mut backlog = vec![];
        while let Some(y) = angles.pop_front() {
            if x.0 == y.0 {
                backlog.push(y);
            } else {
                angles.push_front(y);
                break;
            }
        }
        for x in backlog {
            angles.push_back(x);
        }
    }
    println!("{:?}", dead[199]);
}

fn count_visible(map: &HashSet<(usize, usize)>, from: (usize, usize)) -> Vec<(usize, usize)> {
    // let mut count = 0;
    let mut ret = Vec::new();
    'next: for &to in map.iter() {
        if to == from {
            continue;
        }
        for p in line_of_slight(to, from) {
            if map.contains(&p) {
                continue 'next;
            }
        }
        // println!("{:?} => {:?}", from, to);
        // count += 1;
        ret.push(to);
    }
    // count
    ret.sort_unstable();
    ret
}

fn line_of_slight(from: (usize, usize), to: (usize, usize)) -> Vec<(usize, usize)> {
    // let scale = 1;
    let mut line = vec![];
    let mut x = from.0 as f64;
    let mut y = from.1 as f64;
    let dx = to.0 as f64 - from.0 as f64;
    let dy = to.1 as f64 - from.1 as f64;
    let mut step = 0.0;
    let steps = dx.abs().max(dy.abs());
    // println!(
    //     "from={:?} to={:?} steps={} dx={} dy={}",
    //     from, to, steps, dx, dy,
    // );
    while step < steps {
        step += 1.0;
        x += dx / steps;
        y += dy / steps;
        // println!("x={} y={}", x, y);
        if (x.round() - x).abs() < 1e-6 && (y.round() - y).abs() < 1e-6 {
            line.push((x.round() as usize, y.round() as usize));
        }
    }
    line.pop();
    line
}

fn angle_of(from: (usize, usize), to: (usize, usize)) -> (f64, f64) {
    if from == to {
        return (0.0, 0.0);
    }
    let dy = to.0 as f64 - from.0 as f64;
    let dx = from.1 as f64 - to.1 as f64;
    let angle = dy.atan2(dx);
    let angle = angle.to_degrees();
    let angle = if angle < 0.0 { angle + 360.0 } else { angle };
    (angle, (dx * dx + dy * dy).sqrt())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_line_of_slight() {
        assert_eq!(
            super::line_of_slight((0, 0), (5, 5)),
            vec![(1, 1), (2, 2), (3, 3), (4, 4),]
        );
        assert_eq!(
            super::line_of_slight((5, 5), (0, 0)),
            vec![(4, 4), (3, 3), (2, 2), (1, 1),]
        );
        assert_eq!(
            super::line_of_slight((5, 0), (0, 5)),
            vec![(4, 1), (3, 2), (2, 3), (1, 4),]
        );
        assert_eq!(
            super::line_of_slight((5, 0), (0, 10)),
            vec![(4, 2), (3, 4), (2, 6), (1, 8),]
        );
    }

    #[test]
    fn test_angle_of() {
        assert_eq!(super::angle_of((1, 1), (1, 1)), (0.0, 0.0));
        assert_eq!(super::angle_of((1, 1), (1, 0)), (0.0, 1.0));
        assert_eq!(super::angle_of((1, 1), (2, 1)), (90.0, 1.0));
        assert_eq!(super::angle_of((1, 1), (1, 2)), (180.0, 1.0));
        assert_eq!(super::angle_of((1, 1), (0, 1)), (270.0, 1.0));
    }
}
