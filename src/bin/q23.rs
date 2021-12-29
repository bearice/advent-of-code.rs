use std::collections::VecDeque;

use advent_of_code::common::{Program, ProgramOutput};
use itertools::Itertools;

fn main() {
    let code = Program::from_file("input23.txt");
    let mut rx = (0..50).map(|x| VecDeque::from([x])).collect_vec();
    let mut tx = (0..50).map(|_| VecDeque::new()).collect_vec();
    let mut mem = (0..50).map(|_| code.clone()).collect_vec();
    let mut first = None;
    let mut nat = None;
    let mut last = None;
    loop {
        for buf in tx.iter_mut() {
            // println!("tx[{}]: {:?}", i, buf);
            while buf.len() >= 3 {
                let dst = buf.pop_front().unwrap() as usize;
                let x = buf.pop_front().unwrap();
                let y = buf.pop_front().unwrap();
                if dst == 255 {
                    if first.is_none() {
                        first = Some((x, y));
                    }
                    nat = Some((x, y));
                } else {
                    rx[dst].push_back(x);
                    rx[dst].push_back(y);
                }
            }
        }
        let mut has_io = false;
        for (i, code) in mem.iter_mut().enumerate() {
            while let ProgramOutput::Output(x) = run_with_inputs(code, || {
                has_io |= !rx[i].is_empty();
                rx[i].pop_front().or(Some(-1))
            }) {
                has_io |= true;
                tx[i].push_back(x);
            }
        }
        if !has_io {
            let (x, y) = nat.unwrap();
            rx[0].push_back(x);
            rx[0].push_back(y);
            println!("sent nat pkt: {} {} {}", 0, x, y);
            if last.is_some() && last.unwrap() == (x, y) {
                break;
            } else {
                last = Some((x, y));
            }
        }
    }
    println!("first: {:?}", first);
    println!("last: {:?}", last);
}

pub fn run_with_inputs(code: &mut Program, inputs: impl FnOnce() -> Option<i64>) -> ProgramOutput {
    let input = match code.run_program(None) {
        ProgramOutput::NeedInput => inputs(),
        x => return x,
    };
    code.run_program(input)
}
