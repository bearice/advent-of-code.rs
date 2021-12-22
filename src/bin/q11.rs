use std::collections::HashMap;

use advent_of_code::common::{find_edge, read_lines, Program, ProgramOutput};

fn main() {
    let codes = read_lines("./input11.txt")
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let program = Program::new(codes);
    let mut panels = HashMap::new();
    run_robot(&mut panels, program.clone());
    println!("{}", panels.len());

    let mut panels = HashMap::new();
    panels.insert((0, 0), 1);
    run_robot(&mut panels, program);
    draw_points(panels);
}

fn draw_points(points: HashMap<(i32, i32), i64>) {
    let (min_x, max_x, min_y, max_y) = find_edge(points.keys().cloned());
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if points.get(&(x, y)).unwrap_or(&0) == &1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn run_robot(panels: &mut HashMap<(i32, i32), i64>, mut program: Program) {
    let mut dir = 0;
    let mut pos = (0, 0);
    loop {
        let color = panels.entry(pos).or_insert(0);
        let new_color = program.run_program(Some(*color));
        let new_color = if let ProgramOutput::Output(new_color) = new_color {
            new_color
        } else {
            // println!("{:?}", new_color);
            break;
        };

        let new_dir = if let ProgramOutput::Output(new_dir) = program.run_program(None) {
            new_dir
        } else {
            panic!("unexpected output");
        };
        *color = new_color;
        dir += if new_dir == 0 { 3 } else { 1 };
        dir %= 4;
        match dir {
            0 => pos.1 -= 1,
            1 => pos.0 += 1,
            2 => pos.1 += 1,
            3 => pos.0 -= 1,
            _ => panic!("Invalid direction"),
        }
    }
}
