use std::{collections::HashMap, ops::Shl};

use advent_of_code::common::{find_edge, read_lines};

fn main() {
    let mut lines = read_lines("input20.txt");

    let pattern = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == '.' { 0 } else { 1 })
        .collect::<Vec<_>>();

    lines.next();

    let image = lines
        .enumerate()
        .flat_map(|(y, s)| {
            s.char_indices()
                .map(|(x, c)| {
                    if c == '.' {
                        ((x as i32, y as i32), 0)
                    } else {
                        ((x as i32, y as i32), 1)
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<_, _>>();

    // print_map(&image);

    let image1 = enhance_n(&image, &pattern, 2);
    // print_map(&image1);

    println!("{}", image1.into_values().filter(|&v| v > 0).count());

    let image2 = enhance_n(&image, &pattern, 50);
    // print_map(&image1);

    println!("{}", image2.into_values().filter(|&v| v > 0).count());
}

fn enhance_n(image: &HashMap<(i32, i32), u8>, pattern: &[u8], n: u8) -> HashMap<(i32, i32), u8> {
    let mut image = image.clone();
    let mut void = 0;
    for _ in 0..n {
        image = enhance(&image, pattern, void);
        void = if void == 0 { pattern[0] } else { pattern[511] };
    }
    image
}

fn enhance(image: &HashMap<(i32, i32), u8>, pattern: &[u8], void: u8) -> HashMap<(i32, i32), u8> {
    let mut new_image = HashMap::new();
    let (min_x, max_x, min_y, max_y) = find_edge(image.keys().cloned());
    let (min_x, max_x, min_y, max_y) = (min_x - 1, max_x + 1, min_y - 1, max_y + 1);
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let i = adjacent(x, y, image, void)
                .into_iter()
                .fold(0usize, |acc, x| acc.shl(1) + x as usize);
            new_image.insert((x, y), pattern[i]);
        }
    }
    new_image
}

fn adjacent(x: i32, y: i32, image: &HashMap<(i32, i32), u8>, void: u8) -> Vec<u8> {
    let mut ret = Vec::new();
    for p in [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ] {
        if let Some(&v) = image.get(&p) {
            ret.push(v);
        } else {
            ret.push(void);
        }
    }
    ret
}
