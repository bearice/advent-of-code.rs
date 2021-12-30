use std::{collections::HashMap, rc::Rc};

use advent_of_code::common::{Program, ProgramOutput};
use itertools::Itertools;

fn main() {
    let mut code = AsciiComputer::new("input25.txt");
    // manual_play(&mut code);
    let room0 = Rc::new(Room::new(vec![], code.run("").unwrap()));
    let mut rooms = HashMap::from([(room0.name.to_owned(), room0.clone())]);
    search_room(&mut code, &mut rooms, &room0.name, vec![]);

    let inv = code.run("inv").unwrap();
    let inv = inv
        .lines()
        .filter_map(|l| l.strip_prefix("- "))
        .collect_vec();

    println!("items: {:?}", inv);

    let check_point = rooms["Security Checkpoint"].clone();
    // goto Security Checkpoint
    for d in check_point.path.iter() {
        code.run(d).unwrap();
    }

    let dir = oppsite_dir(check_point.path.last().unwrap());
    let dir = check_point.doors.iter().find(|&x| x != dir).unwrap();

    for set in inv.iter().powerset() {
        let inv = code.run("inv").unwrap();
        let inv = inv
            .lines()
            .filter_map(|l| l.strip_prefix("- "))
            .collect_vec();
        for i in &inv {
            if set.contains(&i) {
                continue;
            }
            code.run(&format!("drop {}", i)).unwrap();
        }
        for i in set {
            if inv.contains(i) {
                continue;
            }
            code.run(&format!("take {}", i)).unwrap();
        }
        let out = code.run(dir).unwrap();
        if !out.contains("you are ejected back") {
            println!("{}", out);
            break;
        }
    }
}

// fn manual_play(code: &mut AsciiComputer) {
//     let mut input = String::new();
//     while let Some(output) = code.run(&input) {
//         println!("{}", output);
//         print!("input>");
//         input.clear();
//         std::io::stdin().read_line(&mut input).unwrap();
//         input.pop();
//         input.pop();
//     }
// }

fn search_room(
    code: &mut AsciiComputer,
    rooms: &mut HashMap<String, Rc<Room>>,
    name: &str,
    path: Vec<String>,
) {
    let room = rooms.get(name).unwrap().clone();
    println!("searching room: {:?}", room.name);
    println!("{}", room.desc);

    const BLACKLIST: &[&str] = &[
        "infinite loop",
        "photons",
        "giant electromagnet",
        "molten lava",
        "escape pod",
    ];
    for item in &room.items {
        if BLACKLIST.contains(&item.as_str()) {
            continue;
        }
        println!("take {} from {:?}", item, name);
        code.run(&format!("take {}", item)).unwrap();
    }
    // println!("items: {}", code.run("inv").unwrap());

    for dir in &room.doors {
        // println!("moving to {} ", dir);
        let last = path.last();
        if last.is_some() && oppsite_dir(dir) == last.unwrap() {
            continue;
        }
        let mut path = path.clone();
        path.push(dir.to_owned());

        let new_room = Room::new(path.clone(), code.run(dir).unwrap());
        if new_room.name.is_empty() {
            continue;
        }
        let name = new_room.name.to_owned();
        rooms.insert(new_room.name.to_owned(), new_room.into());
        if name == "Security Checkpoint" {
            continue;
        }

        search_room(code, rooms, &name, path);
        code.run(oppsite_dir(dir)).unwrap();
    }
}

fn oppsite_dir(dir: &str) -> &str {
    match dir {
        "north" => "south",
        "south" => "north",
        "west" => "east",
        "east" => "west",
        _ => panic!("invalid direction: {}", dir),
    }
}

struct AsciiComputer {
    code: Program,
}

impl AsciiComputer {
    fn new(file: &str) -> AsciiComputer {
        let code = Program::from_file(file);
        AsciiComputer { code }
    }

    fn run(&mut self, input: &str) -> Option<String> {
        // println!("input: {}", input);
        let mut input = input.chars().map(|x| x as i64).collect_vec();
        if !input.is_empty() {
            input.push(10);
        }
        let mut idx = 0;
        let mut output = String::new();
        let mut feed = None;
        loop {
            match self.code.run_program(feed) {
                ProgramOutput::NeedInput => {
                    if idx == input.len() {
                        return Some(output);
                    }
                    feed = input.get(idx).cloned();
                    idx += 1;
                }
                ProgramOutput::Output(x) => {
                    output.push((x as u8) as char);
                    feed = None;
                }
                ProgramOutput::Halt => {
                    if output.is_empty() {
                        return None;
                    } else {
                        return Some(output);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Room {
    name: String,
    desc: String,
    doors: Vec<String>,
    items: Vec<String>,
    path: Vec<String>,
}

impl Room {
    fn new(path: Vec<String>, desc: String) -> Self {
        let mut lines = desc.lines();
        let mut desc = String::new();
        let mut name = String::new();
        let mut doors = vec![];
        let mut items = vec![];
        while let Some(line) = lines.next() {
            if line == "Doors here lead:" {
                for line in &mut lines {
                    if line.is_empty() {
                        break;
                    }
                    doors.push(line[2..].to_string());
                }
            } else if line == "Items here:" {
                for line in &mut lines {
                    if line.is_empty() {
                        break;
                    }
                    items.push(line[2..].to_string());
                }
            } else if line.starts_with("==") {
                name = line[3..line.len() - 3].to_string();
            } else if line != "Command?" && !line.is_empty() {
                desc = line.to_string();
            }
        }
        Room {
            name,
            desc,
            doors,
            items,
            path,
        }
    }
}
