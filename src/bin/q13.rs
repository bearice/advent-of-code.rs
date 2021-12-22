use std::collections::HashSet;

use advent_of_code::common::{Program, ProgramOutput};

fn main() {
    let mut code = Program::from_file("input13.txt");
    {
        let mut code = code.clone();
        let mut outputs = Vec::new();
        loop {
            match code.run_program(None) {
                ProgramOutput::Output(x) => outputs.push(x),
                ProgramOutput::Halt => break,
                _ => panic!("Unexpected program output"),
            }
        }
        println!("{}", outputs.chunks(3).filter(|x| x[2] == 2).count());
    }
    let mut outputs = Vec::new();
    code.codes[0] = 2;
    let mut input = None;
    let mut bricks = HashSet::new();
    loop {
        // println!("pc={:?}", code.pc);
        match code.run_program(input) {
            ProgramOutput::Output(x) => {
                // println!("output: {}", x);
                outputs.push(x);
                input = None;
            }
            ProgramOutput::Halt => {
                println!("halt: pc={}", code.pc);
                break;
            }
            ProgramOutput::NeedInput => {
                // println!("out={:?}", outputs.len());
                input = find_input(&outputs, &mut bricks);
                println!("in={:?}", input);
                outputs.clear();
            }
        }
    }
    find_input(&outputs, &mut bricks);
    println!("{:?}", bricks);
}

fn find_input(outputs: &[i64], bricks: &mut HashSet<(i64, i64)>) -> Option<i64> {
    let mut ball = (0, 0);
    let mut paddle = (0, 0);
    for ch in outputs.chunks(3) {
        let x = ch[0];
        let y = ch[1];
        let color = ch[2];
        if x == -1 && y == 0 {
            println!("Score: {}", ch[2]);
        } else {
            match color {
                0 => {
                    bricks.remove(&(x, y));
                }
                2 => {
                    bricks.insert((x, y));
                }
                3 => paddle = (x, y),
                4 => ball = (x, y),
                _ => (),
            }
        }
    }
    println!(
        "ball:{:?} paddle:{:?} bricks:{}",
        ball,
        paddle,
        bricks.len()
    );
    Some((ball.0 - paddle.0).signum() as i64)
}
