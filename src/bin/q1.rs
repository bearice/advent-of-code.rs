use advent_of_code::common::read_lines;
fn main() {
    let v: Vec<i32> = read_lines("./input1.txt")
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{}", v.iter().cloned().map(fuel_for_mass).sum::<i32>());
    println!(
        "{}",
        v.iter().cloned().map(fuel_for_mass_recursive).sum::<i32>()
    );
}

fn fuel_for_mass(mass: i32) -> i32 {
    if mass < 6 {
        0
    } else {
        mass / 3 - 2
    }
}

fn fuel_for_mass_recursive(mass: i32) -> i32 {
    let fuel = fuel_for_mass(mass);
    if fuel == 0 {
        0
    } else {
        fuel + fuel_for_mass_recursive(fuel)
    }
}
