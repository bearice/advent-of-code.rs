use advent_of_code::common::read_lines;
fn main() {
    let mut v: Vec<usize> = read_lines("./input1.txt")
        .unwrap()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();
    v.sort();
    // println!("{:?}", v);
    let mut x = 0;
    let mut i = v.iter();
    while let Some(a) = i.next() {
        let mut i2 = i.clone();
        while let Some(b) = i2.next() {
            if a + b >= 2020 {
                break;
            }
            let mut i3 = i2.clone();
            while let Some(c) = i3.next() {
                x = x + 1;
                if a + b + c == 2020 {
                    println!("{}", a * b * c);
                } else if a + b + c > 2020 {
                    break;
                }
            }
        }
    }
    // println!("{},{}", x, v.len());
}
