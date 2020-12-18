use advent_of_code::common::read_lines;

fn slope(x: usize, y: usize, cnt: &mut usize, i: &mut usize, j: &mut usize, line: &Vec<char>) {
    // if y == 2 {
    //     println!(
    //         "y={},j={},j%y={}, i={}, s={}",
    //         y,
    //         j,
    //         *j % y,
    //         i,
    //         line.get(*i).unwrap()
    //     );
    // }
    if *j % y == 0 {
        if line.get(*i).unwrap().eq(&'#') {
            *cnt += 1;
        }
        *i += x;
        *i %= line.len();
    }
    *j += 1;
}

fn main() {
    let mut lines = read_lines("./input3.txt").unwrap();
    let mut i = (0, 0, 0, 0, 0);
    let mut j = (0, 0, 0, 0, 0);
    let mut cnt = (0, 0, 0, 0, 0);

    while let Some(Ok(line)) = lines.next() {
        let a: Vec<char> = line.chars().collect();
        macro_rules! slope {
            ($x:expr, $y:expr, $z:tt ) => {
                slope($x, $y, (&mut cnt.$z), (&mut i.$z), (&mut j.$z), &a);
            };
        }
        slope!(1, 1, 0);
        slope!(3, 1, 1);
        slope!(5, 1, 2);
        slope!(7, 1, 3);
        slope!(1, 2, 4);
    }
    // println!("{:?}", cnt);
    println!("{}", cnt.0 * cnt.1 * cnt.2 * cnt.3 * cnt.4);
}
