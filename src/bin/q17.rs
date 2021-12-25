use std::{collections::HashSet, vec};

use advent_of_code::common::{Program, ProgramOutput};

fn main() {
    let mut code = Program::from_file("input17.txt");
    let mut image = vec![];
    let mut line = vec![];
    let mut code1 = code.clone();
    loop {
        match code1.run_program(None) {
            ProgramOutput::Output(x) => {
                let ch = x as u8;
                if ch != 10 {
                    line.push(ch as char);
                } else {
                    if !line.is_empty() {
                        image.push(line);
                    }
                    line = vec![];
                }
            }
            ProgramOutput::Halt => break,
            _ => panic!("unexpected"),
        }
    }

    let mut corssings = HashSet::new();
    let mut robot = None;
    for y in 1..image.len() - 1 {
        let row = &image[y];
        for x in 1..row.len() - 1 {
            if row[x] == '#'
                && row[x - 1] == '#'
                && row[x + 1] == '#'
                && image[y - 1][x] == '#'
                && image[y + 1][x] == '#'
            {
                corssings.insert((x, y));
            } else if row[x] == '^' {
                robot = Some((x, y));
            }
        }
    }
    println!("{:?}", corssings.iter().map(|(x, y)| x * y).sum::<usize>());
    let mut robot = robot.unwrap();
    let mut instructions = vec![];
    let mut dir = 0;
    while let Some(step) = find_next_step(&mut image, &mut corssings, &mut robot, &mut dir) {
        instructions.push(step);
    }

    let cmds = find_cmds(&instructions);
    let cmd = cmds.join("\n") + "\nn\n";
    let mut inputs = cmd.as_bytes().iter().map(|x| *x as i64);
    let mut input = None;
    code.codes[0] = 2;
    loop {
        match code.run_program(input) {
            ProgramOutput::Output(x) => {
                if x > 128 {
                    println!("{}", x);
                    // } else {
                    //     print!("{}", (x as u8) as char);
                }
                input = None;
            }
            ProgramOutput::NeedInput => {
                input = inputs.next();
            }
            ProgramOutput::Halt => break,
        }
    }
}

fn find_cmds(instructions: &[String]) -> [String; 4] {
    fn find_matches<'a>(
        input: &'a [String],
        patterns: &[&[String]],
    ) -> (&'a [String], Vec<&'static str>) {
        let mut remains = input;
        let mut matches = vec![];
        loop {
            let mut found = false;
            for (n, &patterns) in patterns.iter().enumerate() {
                let name = ["A", "B", "C"][n];
                if remains.starts_with(patterns) {
                    matches.push(name);
                    remains = &remains[patterns.len()..];
                    found = true;
                }
            }
            if !found {
                break;
            }
        }
        (remains, matches)
    }
    fn search<'a>(
        remains: &'a [String],
        patterns: &mut Vec<&'a [String]>,
        matches: Vec<&'static str>,
    ) -> Option<Vec<&'static str>> {
        for len in 1..remains.len() {
            let pat = &remains[0..len];
            if pat.len() > 10 {
                break;
            }
            patterns.push(pat);
            let (new_remains, new_matches) = find_matches(remains, patterns);
            let mut matches = matches.clone();
            matches.extend(new_matches);
            if patterns.len() == 3 {
                if new_remains.is_empty() {
                    return Some(matches);
                }
            } else if let Some(ret) = search(new_remains, patterns, matches) {
                return Some(ret);
            }
            patterns.pop();
        }
        None
    }
    let mut func = vec![];
    let matches = search(instructions, &mut func, vec![]).unwrap();
    [
        matches.join(","),
        func[0].join(","),
        func[1].join(","),
        func[2].join(","),
    ]
}

fn find_next_step(
    map: &mut [Vec<char>],
    crossing: &mut HashSet<(usize, usize)>,
    robot: &mut (usize, usize),
    dir: &mut usize,
) -> Option<String> {
    let max = (map[0].len() - 1, map.len() - 1);
    // if we can move ahead, do it until we have to stop
    if let Some(next) = next_pos(*robot, max, *dir) {
        // println!("ahead: {:?}", next);
        if map[next.1][next.0] == '#' {
            let mut n = 1;
            if crossing.contains(robot) {
                crossing.remove(robot);
            } else {
                map[robot.1][robot.0] = '.';
            }
            *robot = next;
            while let Some(next) = next_pos(*robot, max, *dir) {
                if map[next.1][next.0] == '#' {
                    if crossing.contains(robot) {
                        crossing.remove(robot);
                    } else {
                        map[robot.1][robot.0] = '.';
                    }
                    *robot = next;
                    n += 1;
                } else {
                    break;
                }
            }
            return Some(n.to_string());
        }
    }
    // if we can't move ahead, try turn left
    *dir += 3;
    *dir %= 4;
    if let Some(next) = next_pos(*robot, max, *dir) {
        // println!("left: {:?}", next);
        if map[next.1][next.0] == '#' && !crossing.contains(&next) {
            return Some("L".to_string());
        }
    }
    // try turn right (turn backward because we are facing left)
    *dir += 2;
    *dir %= 4;
    if let Some(next) = next_pos(*robot, max, *dir) {
        // println!("right: {:?}", next);
        if map[next.1][next.0] == '#' && !crossing.contains(&next) {
            return Some("R".to_string());
        }
    }
    // no way to go, we have finished
    None
}

fn next_pos(pos: (usize, usize), max: (usize, usize), dir: usize) -> Option<(usize, usize)> {
    match dir {
        0 => {
            if pos.1 == 0 {
                None
            } else {
                Some((pos.0, pos.1 - 1))
            }
        }
        1 => {
            if pos.0 == max.0 {
                None
            } else {
                Some((pos.0 + 1, pos.1))
            }
        }
        2 => {
            if pos.1 == max.1 {
                None
            } else {
                Some((pos.0, pos.1 + 1))
            }
        }
        3 => {
            if pos.0 == 0 {
                None
            } else {
                Some((pos.0 - 1, pos.1))
            }
        }
        _ => panic!("invalid direction"),
    }
}
