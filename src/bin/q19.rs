use std::collections::HashMap;

use advent_of_code::common::{Program, ProgramOutput};

fn main() {
    let code = Program::from_file("input19.txt");
    println!("{}", solve1(&code));
    println!("{}", solve2(&code));
}

fn solve1(code: &Program) -> i64 {
    let mut i = 0;
    for x in 0..50 {
        for y in 0..50 {
            if check_pos(code, x, y) > 0 {
                i += 1;
            }
        }
    }
    i
}

fn solve2(code: &Program) -> i64 {
    let delta = 99;
    let mut cache = HashMap::new();
    let mut y1 = 1000;
    let mut y2 = y1 + delta;
    let mut x1 = find_edge(code, &mut cache, y1).1;
    let mut x2 = find_edge(code, &mut cache, y2).0;

    loop {
        println!("x1: {:?} x2: {:?}  x1-x2:{}", x1, x2, x1 - x2);
        if x1 - x2 >= delta {
            break;
        }
        y1 += 1;
        y2 += 1;
        x1 = find_edge(code, &mut cache, y1).1;
        x2 = find_edge(code, &mut cache, y2).0;
    }
    println!("chk({},{})={}", x1, y1, check_pos(code, x1, y1));
    println!("chk({},{})={}", x2, y2, check_pos(code, x2, y2));
    x2 * 10000 + y1
}

fn find_edge(code: &Program, cache: &mut HashMap<i64, (i64, i64)>, y: i64) -> (i64, i64) {
    if cache.contains_key(&y) {
        return cache[&y];
    }
    let mut x = if let Some(e) = cache.get(&(y - 1)) {
        e.1 + 10
    } else {
        y
    };
    while check_pos(code, x, y) == 0 {
        x -= 1;
    }
    let max = x;
    while check_pos(code, x, y) == 1 {
        x -= 1;
    }
    let min = x + 1;
    let ret = (min, max);
    cache.insert(y, ret);
    ret
}

fn check_pos(code: &Program, x: i64, y: i64) -> i64 {
    let mut code = code.clone();
    let out = code.run_program(Some(x));
    if out != ProgramOutput::NeedInput {
        panic!("unexpected output");
    }
    let out = code.run_program(Some(y));
    if let ProgramOutput::Output(out) = out {
        return out;
    }
    panic!("unexpected output");
}
