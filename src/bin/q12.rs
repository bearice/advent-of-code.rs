use advent_of_code::common::read_lines;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Moon {
    pos: [i32; 3],
    vel: [i32; 3],
}

impl Moon {
    fn from_str(s: String) -> Moon {
        let vel = [0; 3];
        let pos = s[1..s.len() - 1]
            .split(", ")
            .map(|s| s.split_once('=').unwrap().1.parse().unwrap())
            .collect_vec();
        let pos = [pos[0], pos[1], pos[2]];
        Moon { pos, vel }
    }
    fn engry(&self) -> i32 {
        self.pos.iter().map(|x| x.abs()).sum::<i32>()
            * self.vel.iter().map(|x| x.abs()).sum::<i32>()
    }
}

fn tick(moons: &mut [Moon]) {
    (0..moons.len()).tuple_combinations().for_each(|(m1, m2)| {
        for i in 0..3 {
            match moons[m1].pos[i].cmp(&moons[m2].pos[i]) {
                std::cmp::Ordering::Less => {
                    moons[m1].vel[i] += 1;
                    moons[m2].vel[i] -= 1;
                }
                std::cmp::Ordering::Greater => {
                    moons[m1].vel[i] -= 1;
                    moons[m2].vel[i] += 1;
                }
                _ => {}
            }
        }
    });
    for moon in moons {
        for i in 0..3 {
            moon.pos[i] += moon.vel[i];
        }
    }
}

fn q1(moons: Vec<Moon>) -> i32 {
    let mut moons = moons;
    for _ in 0..1000 {
        tick(&mut moons);
    }
    moons.iter().map(|m| m.engry()).sum()
}

fn find_loop(n: Vec<i32>) -> usize {
    // println!("{:?}", n);
    let start = n.iter().map(|x| (*x, 0)).collect_vec();
    let mut state = start.clone();
    let mut cnt = 0;
    while !(cnt > 0 && state == start) {
        (0..n.len()).tuple_combinations().for_each(|(m1, m2)| {
            match state[m1].0.cmp(&state[m2].0) {
                std::cmp::Ordering::Less => {
                    state[m1].1 += 1;
                    state[m2].1 -= 1;
                }
                std::cmp::Ordering::Greater => {
                    state[m1].1 -= 1;
                    state[m2].1 += 1;
                }
                _ => {}
            }
        });
        for i in &mut state {
            i.0 += i.1;
        }
        cnt += 1;
        // println!("{:?}", state);
        // if cnt > 100 {
        //     break;
        // }
    }
    cnt
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn q2(moons: Vec<Moon>) -> usize {
    let mut t = Vec::new();
    for i in 0..3 {
        t.push(find_loop(moons.iter().map(|m| m.pos[i]).collect_vec()));
    }
    t.into_iter().reduce(lcm).unwrap()
}
fn main() {
    let moons = read_lines("input12.txt").map(Moon::from_str).collect_vec();
    println!("{:?}", q1(moons.clone()));
    println!("{:?}", q2(moons));
}
