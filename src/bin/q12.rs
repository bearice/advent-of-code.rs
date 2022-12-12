use advent_of_code::common::{adj_list, read_matrix, shortest_path};

fn main() {
    let mut map = read_matrix("./input12.txt");
    // map.iter().for_each(|x| println!("{:?}", x));
    let size = (map.len(), map[0].len());
    let start = find_and_replace(&mut map, 'S', 'a');
    let end = find_and_replace(&mut map, 'E', 'z');
    println!(
        "{}",
        shortest_path(start, end, |x| paths1(&map, x, &size)).unwrap()
    );
    println!(
        "{}",
        shortest_path(start, end, |x| paths2(&map, x, &size)).unwrap()
    );
}

fn paths1(
    map: &[Vec<char>],
    pos: &(usize, usize),
    size: &(usize, usize),
) -> Vec<((usize, usize), usize)> {
    let max = map[pos.0][pos.1] as u8 + 1;
    let adj = adj_list(*pos, *size);
    adj.into_iter()
        .filter(|pos| map[pos.0][pos.1] as u8 <= max)
        .map(|x| (x, 1))
        .collect()
}

fn paths2(
    map: &[Vec<char>],
    pos: &(usize, usize),
    size: &(usize, usize),
) -> Vec<((usize, usize), usize)> {
    let max = map[pos.0][pos.1] as u8 + 1;
    let adj = adj_list(*pos, *size);
    adj.into_iter()
        .filter(|pos| map[pos.0][pos.1] as u8 <= max)
        .map(|x| (x, if map[x.0][x.1] == 'a' { 0 } else { 1 }))
        .collect()
}

fn find_and_replace(map: &mut [Vec<char>], from: char, to: char) -> (usize, usize) {
    for i in 0..map.len() {
        let row = &mut map[i];
        for j in 0..row.len() {
            if row[j] == from {
                row[j] = to;
                return (i, j);
            }
        }
    }
    panic!()
}
