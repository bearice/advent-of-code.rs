use advent_of_code::common::read_lines;
fn main() {
    let mut numbers: Vec<usize> = read_lines("./input10.txt")
        .unwrap()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();

    numbers.push(0);
    numbers.sort();
    let mut i = numbers.iter();
    i.next();
    let mut diff: Vec<_> = numbers.iter().zip(i).map(|x| x.1 - x.0).collect();
    println!("{:?}", diff);
    let n = diff.iter().fold((0, 1), |mut acc, x| {
        match *x {
            1 => acc.0 += 1,
            3 => acc.1 += 1,
            _ => (),
        };
        acc
    });
    diff.push(0);
    diff.push(0);
    diff.push(0);
    println!("{:?}", n.0 * n.1);
    let mut cnt = 0;
    for i in 0..diff.len() - 3 {
        if diff[i] == 1 && diff[i + 1] == 1 {
            cnt += 1;
            if diff[i] == 1 && diff[i + 2] == 1 {
                cnt += 1;
                if diff[i] == 1 && diff[i + 3] == 1 {
                    cnt += 1;
                }
            }
        }
    }
    println!("{}", cnt);
}
