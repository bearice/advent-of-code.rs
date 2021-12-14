use std::vec;

use advent_of_code::common::read_lines;

fn main() {
    let input = read_lines("input8.txt").next().unwrap();
    let mut numbers = input.as_bytes().iter().map(|&x| x - 48);
    let mut layers = vec![];
    while let Some(x) = read_image((25, 6), &mut numbers) {
        layers.push(x);
    }
    let min_layer = layers.iter().min_by_key(|&x| count_digits(x, 0)).unwrap();
    let ones = count_digits(min_layer, 1);
    let twos = count_digits(min_layer, 2);
    println!("{}", ones * twos);

    let mut image = vec![2; 25 * 6];
    for layer in layers {
        for (i, pixel) in layer.iter().enumerate() {
            if image[i] == 2 {
                image[i] = *pixel;
            }
        }
    }
    for row in image.chunks(25) {
        for pixel in row {
            print!(
                "{}",
                match pixel {
                    0 => ' ',
                    1 => '#',
                    _ => panic!(),
                }
            );
        }
        println!();
    }
}

fn count_digits(layer: &[u8], digit: u8) -> usize {
    layer.iter().filter(|&&x| x == digit).count()
}

fn read_image(size: (usize, usize), input: &mut impl Iterator<Item = u8>) -> Option<Vec<u8>> {
    let mut image = vec![];
    for _ in 0..size.0 * size.1 {
        image.push(input.next()?);
    }
    Some(image)
}
