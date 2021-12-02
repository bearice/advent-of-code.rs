use advent_of_code::common::read_lines;
fn main() {
    let v: Vec<i32> = read_lines("./input1.txt")
        .map(|x| x.parse().unwrap())
        .collect();

    fn count_incr(v: impl Iterator<Item = i32>) -> i32 {
        v.fold((-1, 0), |(n, last), x| {
            // println!("n={} last={} x={}", n, last, x);
            if n == -1 {
                (0, x)
            } else if x > last {
                (n + 1, x)
            } else {
                (n, x)
            }
        })
        .0
    }
    let a1 = count_incr(v.iter().copied());
    println!("a1={}", a1);
    let v1 = v.iter();
    let v2 = v.iter().zip(v1.skip(1));
    let v3 = v.iter().zip(v2.skip(1));
    let vs = v3.map(|(x, (y, z))| x + y + z);
    let a2 = count_incr(vs);
    println!("a2={}", a2);
}
