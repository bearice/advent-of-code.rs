use advent_of_code::common::read_lines;

fn main() {
    let mut fishes = [0u64; 9];
    read_lines("./input6.txt")
        .next()
        .unwrap()
        .split(',')
        .for_each(|s| fishes[s.parse::<usize>().unwrap()] += 1);

    for x in 0..256 {
        next_day(&mut fishes);
        if x == 80 {
            println!("day80={}", fishes.iter().sum::<u64>());
        }
    }
    println!("day256={}", fishes.iter().sum::<u64>());
}

fn next_day(fishes: &mut [u64; 9]) {
    let new_fish = fishes[0];
    for i in 1..=8 {
        fishes[i - 1] = fishes[i];
    }
    fishes[6] += new_fish;
    fishes[8] = new_fish;
}
