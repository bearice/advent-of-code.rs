use itertools::Itertools;
use std::cmp::Ordering;
fn main() {
    // let target = (20, 30, -10, -5);
    let target = (156, 202, -110, -69);
    let x_rng = find_x_range(target.0, target.1);
    // println!("x_rng: {:?}", x_rng);
    let y_rng = find_y_range(target.2, target.3);
    // println!("y_rng: {:?}", y_rng);
    let mut common = vec![];
    for &x in x_rng.iter() {
        for &y in y_rng.iter() {
            if x.0 == y.0 || (x.2 == 0 && y.0 > x.0) {
                common.push((x.0, x.1, y.1, y.2));
            }
        }
    }
    println!("max: {}", common.iter().map(|&x| x.3).max().unwrap());
    let uniq = common.iter().map(|&x| (x.1, x.2)).unique();
    println!("uniq: {}", uniq.count());
}

fn find_x_range(start: i32, end: i32) -> Vec<(i32, i32, i32)> {
    let mut ret = vec![];
    for v in 1..=end {
        let mut x = 0;
        let mut vx = v;
        let mut i = 0;
        while x <= end && vx != 0 {
            i += 1;
            x += vx;
            vx += match vx.cmp(&0) {
                Ordering::Less => 1,
                Ordering::Greater => -1,
                Ordering::Equal => 0,
            };
            if x >= start && x <= end {
                ret.push((i, v, vx));
            }
        }
    }
    // ret.sort_unstable();
    ret
}

fn find_y_range(start: i32, end: i32) -> Vec<(i32, i32, i32)> {
    let mut ret = vec![];
    for v in start..=start.abs() {
        let mut y = 0;
        let mut vy = v;
        let mut i = 0;
        let mut max = 0;
        while y >= start {
            i += 1;
            y += vy;
            vy -= 1;
            max = max.max(y);
            if y >= start && y <= end {
                ret.push((i, v, max));
            }
        }
    }
    // ret.sort_unstable();
    ret
}
