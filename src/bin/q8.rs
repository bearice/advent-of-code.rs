use std::iter::repeat;

use advent_of_code::common::read_lines;
use itertools::Itertools;

fn main() {
    let mut input = read_lines("input8.txt")
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .zip(repeat(false))
                .collect_vec()
        })
        .collect_vec();

    for _ in 0..3 {
        check_visible(&mut input);
        rotate_vec(&mut input);
    }
    check_visible(&mut input);
    let count: usize = input
        .iter()
        .map(|row| row.iter().filter(|(_, visible)| *visible).count())
        .sum();
    println!("{}", count);
    let mut max = 0;
    for x in 0..input.len() {
        for y in 0..input.len() {
            max = max.max(score(&input, x, y));
        }
    }
    println!("{}", max);
}

fn check_visible(input: &mut Vec<Vec<(i32, bool)>>) {
    for row in input {
        let mut max = -1;
        for cell in row {
            if cell.0 > max {
                max = cell.0;
                cell.1 = true;
            }
        }
    }
}

fn rotate_vec(vec: &mut Vec<Vec<(i32, bool)>>) {
    let n = vec.len();
    for i in 0..n / 2 {
        for j in i..n - i - 1 {
            let tmp = vec[i][j];
            vec[i][j] = vec[n - j - 1][i];
            vec[n - j - 1][i] = vec[n - i - 1][n - j - 1];
            vec[n - i - 1][n - j - 1] = vec[j][n - i - 1];
            vec[j][n - i - 1] = tmp;
        }
    }
}

fn score(input: &[Vec<(i32, bool)>], x: usize, y: usize) -> usize {
    let n = input.len();
    if x == 0 || y == 0 || x == n - 1 || y == n - 1 {
        return 0;
    }
    let l = count_visible(input, x, y, -1, 0);
    let r = count_visible(input, x, y, 1, 0);
    let u = count_visible(input, x, y, 0, -1);
    let d = count_visible(input, x, y, 0, 1);
    l * r * u * d
}

fn count_visible(input: &[Vec<(i32, bool)>], x: usize, y: usize, dx: i32, dy: i32) -> usize {
    let n = input[x][y].0;
    let mut x = x as i32;
    let mut y = y as i32;
    let mut step = 0;
    loop {
        x += dx;
        y += dy;
        step += 1;
        if input[x as usize][y as usize].0 >= n {
            return step;
        }
        if x == 0 || y == 0 || x as usize == input.len() - 1 || y as usize == input.len() - 1 {
            return step;
        }
    }
}
