use advent_of_code::common::read_u8_matrix;

type Pool = [Vec<u8>];

fn adj(pool: &Pool, ret: &mut Vec<(usize, usize)>, x: usize, y: usize) {
    let x = x as i32;
    let y = y as i32;
    // let mut ret = vec![];
    let mut push_adj = |x: i32, y: i32| {
        if x >= 0 && y >= 0 && y < pool.len() as i32 && x < pool[y as usize].len() as i32 {
            ret.push((x as usize, y as usize));
        }
    };
    push_adj(x - 1, y - 1);
    push_adj(x - 1, y);
    push_adj(x - 1, y + 1);
    push_adj(x, y - 1);
    push_adj(x, y + 1);
    push_adj(x + 1, y - 1);
    push_adj(x + 1, y);
    push_adj(x + 1, y + 1);
}

fn run_round(pool: &mut Pool) -> usize {
    let mut ret = 0;
    let mut queue = vec![];
    for y in 0..pool.len() {
        for x in 0..pool[y].len() {
            let cell = &mut pool[y][x];
            *cell += 1;
            if *cell > 9 {
                *cell = 0;
                ret += 1;
                adj(pool, &mut queue, x, y);
            }
        }
    }
    while let Some((x, y)) = queue.pop() {
        let cell = &mut pool[y][x];
        if *cell > 0 {
            *cell += 1
        }
        if *cell > 9 {
            *cell = 0;
            ret += 1;
            adj(pool, &mut queue, x, y);
        }
    }
    ret
}
fn main() {
    let mut pool = read_u8_matrix("./input11.txt");

    let mut sum = 0;
    let mut i = 0;
    loop {
        sum += run_round(&mut pool);
        // for row in pool.iter() {
        //     println!("{:?}", row);
        // }
        i += 1;
        if i == 100 {
            println!("sum={}", sum);
        }
        if pool.iter().all(|row| row.iter().all(|cell| *cell == 0)) {
            println!("round={}", i);
            break;
        }
    }
}
