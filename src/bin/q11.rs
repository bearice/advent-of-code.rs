use advent_of_code::common::read_lines;

type Seats = Vec<Vec<char>>;
type SeatAdj = Vec<Vec<Vec<(usize, usize)>>>;

fn get_seats<'a>(seats: &'a Seats, (x, y): (usize, usize)) -> Option<&'a char> {
    seats.get(y).and_then(|l| l.get(x))
}

fn is_occupied(x: &&char) -> bool {
    *x == &'#'
}

fn round(seats: &mut Seats, adjs: &SeatAdj, threshold: usize) -> usize {
    let mut ret = 0;
    let snapshot = seats.clone();
    // println!("Round Begin");

    for y in 0..seats.len() {
        for x in 0..seats[y].len() {
            if let Some(s) = get_seats(&snapshot, (x, y)) {
                let adjs = adjs[y][x]
                    .iter()
                    .map(|p| {
                        let ret = get_seats(&snapshot, *p);
                        // if x == 0 && y == 1 {
                        //     println!("p={:?} ret={:?}", p, ret);
                        // }
                        ret
                    })
                    .filter_map(|x| x);
                let cnt = adjs.filter(is_occupied).count();
                //print!("x={} y={} cnt={} ", x, y, cnt);
                match *s {
                    'L' => {
                        if cnt == 0 {
                            seats[y][x] = '#';
                            ret += 1;
                        }
                    }
                    '#' => {
                        if cnt >= threshold {
                            seats[y][x] = 'L';
                            ret += 1;
                        }
                    }
                    _ => (),
                }
                // print!("{}{}{}|", s, cnt, seats[y][x]);
            }
        }
        // println!("");
    }
    ret
}

fn part1_adj(_: &Seats, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut ret = vec![(x, y + 1), (x + 1, y), (x + 1, y + 1)];
    if x > 0 {
        ret.push((x - 1, y));
        ret.push((x - 1, y + 1));
    }
    if y > 0 {
        ret.push((x, y - 1));
        ret.push((x + 1, y - 1));
    }
    if x > 0 && y > 0 {
        ret.push((x - 1, y - 1))
    }
    ret
}
fn part2_adj(seats: &Seats, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    fn try_dir(seats: &Seats, x: usize, y: usize, dx: isize, dy: isize) -> Option<(usize, usize)> {
        let (tx, ty) = (x as isize + dx, y as isize + dy);
        if tx < 0 || ty < 0 {
            None
        } else if let Some(ch) = get_seats(seats, (tx as usize, ty as usize)) {
            // println!("dx={} dy={} tx={} ty={} ch={}", dx, dy, tx + 1, ty + 1, ch);
            if *ch == 'L' {
                // println!("ret={},{}", tx, ty);
                Some((tx as usize, ty as usize))
            } else {
                try_dir(seats, tx as usize, ty as usize, dx, dy)
            }
        } else {
            None
        }
    }
    let ret = vec![
        try_dir(seats, x, y, -1, -1),
        try_dir(seats, x, y, -1, 0),
        try_dir(seats, x, y, -1, 1),
        try_dir(seats, x, y, 0, -1),
        try_dir(seats, x, y, 0, 1),
        try_dir(seats, x, y, 1, -1),
        try_dir(seats, x, y, 1, 0),
        try_dir(seats, x, y, 1, 1),
    ];

    ret.into_iter().filter_map(|x| x).collect()
}

fn run_rounds<F>(mut seats: Seats, adj_fn: F, threshold: usize) -> usize
where
    F: Fn(&Seats, (usize, usize)) -> Vec<(usize, usize)>,
{
    let mut adjs = Vec::with_capacity(seats.len());
    for y in 0..seats.len() {
        let mut row = Vec::with_capacity(seats[y].len());
        for x in 0..seats[y].len() {
            let cell = adj_fn(&seats, (x, y));
            row.push(cell);
        }
        adjs.push(row);
    }

    println!("ajd[1][9]={:?}", adjs[1][9]);
    let mut l = 0;
    let mut n = round(&mut seats, &adjs, threshold);
    while l != n {
        l = n;
        n = round(&mut seats, &adjs, threshold);
    }
    seats
        .into_iter()
        .map(|l| l.iter().filter(is_occupied).count())
        .sum()
}
fn main() {
    let seats: Seats = read_lines("./input11.txt")
        .unwrap()
        .map(Result::unwrap)
        .map(|s| s.chars().collect())
        .collect();

    // println!("{:?}", part2_adj(&seats, (3, 4)));
    println!("p1={}", run_rounds(seats.clone(), part1_adj, 4));
    println!("p2={}", run_rounds(seats.clone(), part2_adj, 5));
}
