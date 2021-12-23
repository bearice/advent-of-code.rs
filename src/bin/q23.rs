use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs::read_to_string,
    hash::Hash,
    ops::{Index, IndexMut},
};

/*
#00 01 02 03 04 05 06 07 08 09 10#
###### 11 ## 12 ## 13 ## 14 ######
    ## 15 ## 16 ## 17 ## 18 ##
    ## 19 ## 20 ## 21 ## 22 ##
    ## 23 ## 24 ## 25 ## 26 ##
*/
fn main() {
    let mut state = [0u8; 19];
    let input = read_to_string("input23.txt").unwrap();
    let mut input = input.chars().filter_map(|c| match c {
        'A' => Some(1),
        'B' => Some(2),
        'C' => Some(3),
        'D' => Some(4),
        _ => None,
    });
    for i in state.iter_mut().skip(11) {
        *i = input.next().unwrap();
    }

    // println!("{:?}", state);
    // let mut test = [0; 27];
    // test[23] = 1;
    // println!("{:?}", engery_of_move(&test, 23, 12));
    // return;

    let end = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
        1, 2, 3, 4, //
        1, 2, 3, 4,
    ];
    let min = shortest_path(state, end, |s| all_moves(s, 19, cell_for_q1));
    println!("{:?}", min.unwrap());

    let iter = [
        &state[0..11],
        &state[11..15],
        &[4, 3, 2, 1],
        &[4, 2, 1, 3],
        &state[15..19],
    ];

    let mut state = [0u8; 27];
    for (i, n) in iter.iter().copied().flatten().copied().enumerate() {
        state[i] = n;
    }

    // println!("{:?}", state);
    let end = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
        1, 2, 3, 4, //
        1, 2, 3, 4, //
        1, 2, 3, 4, //
        1, 2, 3, 4,
    ];
    let min = shortest_path(state, end, |s| all_moves(s, 27, cell_for_q2));
    println!("{:?}", min.unwrap());
}

/*
#00 01 02 03 04 05 06 07 08 09 10#
###### 11 ## 12 ## 13 ## 14 ######
    ## 15 ## 16 ## 17 ## 18 ##
    ## 19 ## 20 ## 21 ## 22 ##
    ## 23 ## 24 ## 25 ## 26 ##
*/
fn engery_of_move<T>(state: &T, mut from: usize, mut to: usize) -> Option<usize>
where
    T: Index<usize, Output = u8>,
{
    // assert!(state[from] > 0);
    // if state[to] != 0 {
    //     // println!("not empty");
    //     return None;
    // }
    let orgi_from = from;
    let mut e = 0;
    let mut checks = vec![];
    if from > 10 {
        from -= 11;
        /*
            # 00 # 01 # 02 # 03 #
            # 04 # 05 # 06 # 07 #
            # 08 # 09 # 10 # 11 #
            # 12 # 13 # 14 # 15 #
        */
        // if from / 4 > 0 {
        //     println!("push 0 {}", from + 11);
        //     checks.push(from + 11);
        // }
        if from / 4 >= 1 {
            // println!("push 1 {}", from - 4 + 11);
            checks.push(from - 4 + 11);
        }
        if from / 4 >= 2 {
            // println!("push 2 {}", from - 8 + 11);
            checks.push(from - 8 + 11);
        }
        if from / 4 >= 3 {
            // println!("push 3 {}", from - 12 + 11);
            checks.push(from - 12 + 11);
        }
        from = (from % 4 + 1) * 2;
        // checks.push(from);
    }
    if to > 10 {
        to -= 11;
        // println!("to:{}", to);
        // if to / 4 == 0 {
        checks.push(to + 11);
        // }
        if to / 4 >= 1 {
            checks.push(to - 4 + 11);
        }
        if to / 4 >= 2 {
            checks.push(to - 8 + 11);
        }
        if to / 4 >= 3 {
            checks.push(to - 12 + 11);
        }
        to = (to % 4 + 1) * 2;
        checks.push(to);
    }
    for i in from.min(to)..to.max(from) {
        checks.push(i);
    }
    // println!("from={} to={} {:?}", from, to, checks);
    for i in checks {
        if state[i] != 0 && i != orgi_from {
            // println!("blocked at {}", i);
            return None;
        }
        e += 10usize.pow((state[orgi_from] - 1) as u32);
    }
    Some(e)
}

fn shortest_path<T, F>(start: T, end: T, edges: F) -> Option<usize>
where
    T: Hash + Eq + Clone + Copy + Ord,
    F: Fn(&T) -> Vec<(T, usize)>,
{
    let mut dist: HashMap<T, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(Reverse((0, start)));

    while let Some(Reverse((cost, pos))) = heap.pop() {
        if pos == end {
            return Some(cost);
        }
        if cost > dist[&pos] {
            continue;
        }
        for (edge, new_cost) in edges(&pos) {
            let new_cost = cost + new_cost;
            let d = dist.entry(edge).or_insert(usize::MAX);
            if new_cost < *d {
                heap.push(Reverse((new_cost, edge)));
                *d = new_cost;
            }
        }
    }
    None
}

fn cell_for_q1(x: u8) -> &'static [usize] {
    match x {
        1 => &[11, 15],
        2 => &[12, 16],
        3 => &[13, 17],
        4 => &[14, 18],
        _ => unreachable!(),
    }
}

fn cell_for_q2(x: u8) -> &'static [usize] {
    match x {
        1 => &[11, 15, 19, 23],
        2 => &[12, 16, 20, 24],
        3 => &[13, 17, 21, 25],
        4 => &[14, 18, 22, 26],
        _ => unreachable!(),
    }
}

/*
#00 01 02 03 04 05 06 07 08 09 10#
###### 11 ## 12 ## 13 ## 14 ######
    ## 15 ## 16 ## 17 ## 18 ##
    ## 19 ## 20 ## 21 ## 22 ##
    ## 23 ## 24 ## 25 ## 26 ##
*/
fn all_moves<T, F>(state: &T, size: usize, cell_for: F) -> Vec<(T, usize)>
where
    T: IndexMut<usize, Output = u8> + Clone + Copy,
    F: Fn(u8) -> &'static [usize],
{
    let mut ret = vec![];
    for from in 0..size {
        if state[from] == 0 {
            continue;
        }
        let targets: &[usize] = if from > 10 {
            &[0, 1, 3, 5, 7, 9, 10]
        } else {
            let cells = cell_for(state[from]);
            if cells
                .iter()
                .any(|&x| state[x] != 0 && state[x] != state[from])
            {
                continue;
            }
            cells
        };
        for &to in targets {
            if from == to || state[to] != 0 {
                continue;
            }

            if let Some(e) = engery_of_move(state, from, to) {
                let mut new_state = *state;
                new_state[to] = new_state[from];
                new_state[from] = 0;
                ret.push((new_state, e));
            }
        }
    }
    ret
}
