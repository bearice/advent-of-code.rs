use advent_of_code::common::read_lines;

type Seats = Vec<Vec<char>>;
fn adjacents_of((x, y): (usize, usize)) -> Vec<(usize, usize)> {
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

fn get_seats<'a>(seats: &'a Seats, (x, y): (usize, usize)) -> Option<&'a char> {
    seats.get(y).and_then(|l| l.get(x))
}

fn is_occupied(x: &&char) -> bool {
    *x == &'#'
}

fn round(seats: &mut Seats) -> usize {
    let mut ret = 0;
    let snapshot = seats.clone();
    // println!("Round Begin");

    for y in 0..seats.len() {
        for x in 0..seats[y].len() {
            if let Some(s) = get_seats(&snapshot, (x, y)) {
                let adjs = adjacents_of((x, y))
                    .into_iter()
                    .map(|p| {
                        let ret = get_seats(&snapshot, p);
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
                        if cnt >= 4 {
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
fn main() {
    let mut lines: Seats = read_lines("./input11.txt")
        .unwrap()
        .map(Result::unwrap)
        .map(|s| s.chars().collect())
        .collect();

    let mut l = 0;
    let mut n = round(&mut lines);
    while l != n {
        l = n;
        n = round(&mut lines);
    }
    let cnt: usize = lines
        .into_iter()
        .map(|l| l.iter().filter(is_occupied).count())
        .sum();
    println!("{}", cnt);
}
