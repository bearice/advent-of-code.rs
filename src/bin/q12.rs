use advent_of_code::common::read_lines;

fn translate1(pos: &mut (isize, isize), dir: &mut isize, cmd: &str, num: isize) {
    match cmd {
        "N" => pos.1 += num,
        "S" => pos.1 -= num,
        "E" => pos.0 += num,
        "W" => pos.0 -= num,
        "F" => translate1(pos, dir, dir_to_cmd(*dir), num),
        "R" => *dir = (*dir + num) % 360,
        "L" => *dir = (*dir + 360 - num) % 360,
        _ => panic!("not possible"),
    }
}

fn translate2(pos: &mut (isize, isize), waypt: &mut (isize, isize), cmd: &str, num: isize) {
    match cmd {
        "N" => waypt.1 += num,
        "S" => waypt.1 -= num,
        "E" => waypt.0 += num,
        "W" => waypt.0 -= num,
        "F" => {
            pos.0 += waypt.0 * num;
            pos.1 += waypt.1 * num;
        }
        "R" => redir_waypt(waypt, num),
        "L" => redir_waypt(waypt, 360 - num),
        _ => panic!("not possible"),
    }
}

fn dir_to_cmd(dir: isize) -> &'static str {
    match dir {
        0 => "N",
        90 => "E",
        180 => "S",
        270 => "W",
        _ => panic!("not possible"),
    }
}

fn redir_waypt(pos: &mut (isize, isize), dir: isize) {
    match dir {
        0 => (),
        90 => *pos = (pos.1, -pos.0),
        180 => *pos = (-pos.0, -pos.1),
        270 => *pos = (-pos.1, pos.0),
        _ => panic!("not possible"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn translate1() {
        let mut pos = (0, 0);
        let mut dir = 0;
        super::translate1(&mut pos, &mut dir, "F", 10);
        assert_eq!(pos, (0, 10));
        assert_eq!(dir, 0);
        super::translate1(&mut pos, &mut dir, "R", 90);
        assert_eq!(pos, (0, 10));
        assert_eq!(dir, 90);
        super::translate1(&mut pos, &mut dir, "F", 10);
        assert_eq!(pos, (10, 10));
        assert_eq!(dir, 90);
        super::translate1(&mut pos, &mut dir, "L", 180);
        assert_eq!(pos, (10, 10));
        assert_eq!(dir, 270);
        super::translate1(&mut pos, &mut dir, "S", 10);
        assert_eq!(pos, (10, 0));
        assert_eq!(dir, 270);
        super::translate1(&mut pos, &mut dir, "W", 10);
        assert_eq!(pos, (0, 0));
        assert_eq!(dir, 270);
    }
}

fn part1(lines: Vec<String>) {
    let mut pos = (0, 0);
    let mut dir = 90;
    for line in lines {
        let (cmd, num) = line.split_at(1);
        let n = num.parse().unwrap();
        translate1(&mut pos, &mut dir, cmd, n);
    }
    println!("{}", pos.0.abs() + pos.1.abs());
}
fn part2(lines: Vec<String>) {
    let mut pos = (0, 0);
    let mut waypt = (10, 1);
    for line in lines {
        let (cmd, num) = line.split_at(1);
        let n = num.parse().unwrap();
        translate2(&mut pos, &mut waypt, cmd, n);
    }
    println!("{}", pos.0.abs() + pos.1.abs());
}
fn main() {
    let lines: Vec<String> = read_lines("./input12.txt").collect();
    part1(lines.clone());
    part2(lines.clone());
}
