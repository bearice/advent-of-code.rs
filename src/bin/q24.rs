use std::collections::HashMap;

use advent_of_code::common::read_lines;

type Pos = (i32, i32);

fn parse_line(line: String) -> Pos {
    let mut ret = (0, 0);
    // println!("{}", line);
    let mut i = line.chars();
    while let Some(c) = i.next() {
        let mut d = "".to_owned();
        d.push(c);
        let odd = ret.1 % 2 == 0;
        match c {
            'e' => ret.0 += 1,
            'w' => ret.0 -= 1,
            'n' => ret.1 += 1,
            's' => ret.1 -= 1,
            _ => panic!("not possible"),
        }
        if c == 'n' || c == 's' {
            let c = i.next().unwrap();
            d.push(c);
            match c {
                'e' => {
                    if odd {
                        ret.0 += 1
                    }
                }
                'w' => {
                    if !odd {
                        ret.0 -= 1
                    }
                }
                _ => panic!("not possible"),
            }
        }
        // println!("{} {:?}", d, ret);
    }
    ret
}
#[cfg(test)]
mod tests {
    #[test]
    fn parse_line() {
        assert_eq!(super::parse_line("eeww".to_owned()), (0, 0));
        assert_eq!(super::parse_line("nwwswee".to_owned()), (0, 0));
        assert_eq!(super::parse_line("esew".to_owned()), (1, -1));
    }
}

fn adjacent(pos: Pos) -> Vec<Pos> {
    let (x, y) = pos;
    if y % 2 == 0 {
        vec![
            (x + 1, y),
            (x - 1, y),
            (x, y + 1),
            (x + 1, y + 1),
            (x, y - 1),
            (x + 1, y - 1),
        ]
    } else {
        vec![
            (x + 1, y),
            (x - 1, y),
            (x, y + 1),
            (x - 1, y + 1),
            (x, y - 1),
            (x - 1, y - 1),
        ]
    }
}

fn mutation(v: Vec<Pos>) -> Vec<Pos> {
    let mut blacks: HashMap<Pos, i32> = v.clone().into_iter().zip(std::iter::repeat(0)).collect();
    let mut whites = HashMap::new();
    for p in v {
        let adjs = adjacent(p);
        for a in adjs.into_iter() {
            if let Some(i) = blacks.get_mut(&a) {
                *i += 1;
            } else if let Some(i) = whites.get_mut(&a) {
                *i += 1;
            } else {
                whites.insert(a, 1);
            }
        }
    }
    let mut ret = vec![];
    for (k, v) in blacks.into_iter() {
        if v == 1 || v == 2 {
            ret.push(k);
        }
    }
    for (k, v) in whites.into_iter() {
        if v == 2 {
            ret.push(k);
        }
    }
    ret
}
fn main() {
    let mut tiles: Vec<_> = read_lines("./input24.txt")
        .unwrap()
        .map(Result::unwrap)
        .map(parse_line)
        .collect();

    tiles.sort();

    // println!("{:?}", tiles);
    let mut last = (tiles[0], 1);
    let mut blacks = Vec::new();
    for i in 1..tiles.len() {
        let t = tiles[i];
        if t == last.0 {
            last.1 += 1;
        } else {
            // println!("{:?}", last);
            if last.1 % 2 == 1 {
                blacks.push(last.0);
            }
            last = (t, 1);
        }
    }
    if last.1 % 2 == 1 {
        blacks.push(last.0);
    }
    println!("{}", blacks.len());

    for i in 0..100 {
        blacks = mutation(blacks);
        println!("{} {}", i + 1, blacks.len());
    }
}
