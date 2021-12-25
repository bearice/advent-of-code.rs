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
        // println!("{:?}", step);
        // println!("pos: {:?}", robot);
        // image[robot.1][robot.0] = 'R';
        // for row in &image {
        //     println!("{}", row.iter().collect::<String>());
        // }
        instructions.push(step);
    }
    // println!("{:?}", instructions);
    // println!("final pos: {:?}", robot);

    let cmds = find_cmds(&instructions);
    let cmd = cmds.join("\n") + "\nn\n";
    let mut inputs = cmd.as_bytes().iter().map(|x| *x as i64);
    let mut input = None;
    code.codes[0] = 2;
    loop {
        match code.run_program(input) {
            ProgramOutput::Output(x) => {
                if x < 128 {
                    print!("{}", (x as u8) as char);
                } else {
                    println!("{}", x);
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
    let mut func = [&instructions[0..]; 3];
    let mut main = None;
    'found: for l1 in 1..instructions.len() {
        func[0] = &instructions[0..l1];
        let mut next1 = &instructions[l1..];
        let mut main1 = vec!["A"];
        loop {
            if next1.starts_with(func[0]) {
                next1 = &next1[func[0].len()..];
                main1.push("A");
            } else {
                break;
            }
        }
        for l2 in 1..next1.len() {
            func[1] = &next1[0..l2];
            if func[1].join(",").len() >= 20 {
                break;
            }
            let mut next2 = &next1[l2..];
            let mut main2 = main1.clone();
            main2.push("B");
            loop {
                if next2.starts_with(func[1]) {
                    next2 = &next2[func[1].len()..];
                    main2.push("B");
                } else if next2.starts_with(func[0]) {
                    next2 = &next2[func[0].len()..];
                    main2.push("A");
                } else {
                    break;
                }
            }
            for l3 in 1..next2.len() {
                func[2] = &next2[0..l3];
                if func[2].join(",").len() >= 20 {
                    break;
                }
                let mut next3 = &next2[l3..];
                let mut main3 = main2.clone();
                main3.push("C");
                loop {
                    if next3.starts_with(func[2]) {
                        next3 = &next3[func[2].len()..];
                        main3.push("C");
                    } else if next3.starts_with(func[1]) {
                        next3 = &next3[func[1].len()..];
                        main3.push("B");
                    } else if next3.starts_with(func[0]) {
                        next3 = &next3[func[0].len()..];
                        main3.push("A");
                    } else {
                        break;
                    }
                }
                if next3.is_empty() {
                    main = Some(main3);
                    break 'found;
                }
            }
        }
    }
    [
        main.unwrap().join(","),
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
