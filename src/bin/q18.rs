use advent_of_code::common::read_lines;
use itertools::Itertools;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
enum SnailFish {
    Pair {
        lv: u32,
        a: Box<SnailFish>,
        b: Box<SnailFish>,
    },
    Regular {
        lv: u32,
        n: usize,
    },
}

impl fmt::Display for SnailFish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnailFish::Pair { a, b, .. } => write!(f, "[{}, {}]", a, b),
            SnailFish::Regular { n, .. } => write!(f, "{}", n),
        }
    }
}

impl SnailFish {
    fn parse(lv: u32, s: &str) -> (SnailFish, &str) {
        // println!("s: {:?}", s);
        if let Some(s) = s.strip_prefix('[') {
            let (a, s) = SnailFish::parse(lv + 1, s);
            let (b, s) = SnailFish::parse(lv + 1, s);
            (
                Self::Pair {
                    lv,
                    a: a.into(),
                    b: b.into(),
                },
                s,
            )
        } else if s.contains(',') {
            let (num, rest) = s.split_once(',').unwrap();
            let n = SnailFish::parse(lv, num);
            (n.0, rest)
        } else if s.contains(']') {
            let (num, rest) = s.split_once(']').unwrap();
            let n = SnailFish::parse(lv, num);
            (n.0, rest)
        } else {
            (
                Self::Regular {
                    lv,
                    n: s.parse().unwrap(),
                },
                "",
            )
        }
    }
    fn magnitude(&self) -> usize {
        match self {
            SnailFish::Pair { a, b, .. } => 3 * a.magnitude() + 2 * b.magnitude(),
            SnailFish::Regular { n, .. } => *n,
        }
    }
    fn add(mut self, mut other: Self) -> Self {
        self.level_up();
        other.level_up();
        let mut ret = Self::Pair {
            lv: 0,
            a: self.into(),
            b: other.into(),
        };
        ret.reduce();
        ret
    }

    fn level_up(&mut self) {
        match self {
            Self::Pair { lv, a, b } => {
                a.level_up();
                b.level_up();
                *lv += 1;
            }
            Self::Regular { lv, .. } => {
                *lv += 1;
            }
        }
    }

    fn reduce(&mut self) {
        loop {
            if self.explode().is_some() {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
    }

    fn as_pair(&mut self) -> Option<(&mut SnailFish, &mut SnailFish)> {
        if let Self::Pair { a, b, .. } = self {
            Some((a, b))
        } else {
            None
        }
    }
    fn as_val(&self) -> usize {
        if let Self::Regular { n, .. } = self {
            *n
        } else {
            panic!("not regular");
        }
    }
    fn add_left(&mut self, n: usize) -> bool {
        if let Self::Regular { n: ref mut m, .. } = self {
            *m += n;
            true
        } else {
            let (a, b) = self.as_pair().unwrap();
            a.add_left(n) || b.add_left(n)
        }
    }
    fn add_right(&mut self, n: usize) -> bool {
        if let Self::Regular { n: ref mut m, .. } = self {
            *m += n;
            true
        } else {
            let (a, b) = self.as_pair().unwrap();
            b.add_right(n) || a.add_right(n)
        }
    }
    fn explode(&mut self) -> Option<(usize, usize)> {
        if let Self::Pair { lv, a, b } = self {
            if *lv >= 4 {
                let ret = (a.as_val(), b.as_val());
                *self = Self::Regular { lv: *lv, n: 0 };
                Some(ret)
            } else if let Some((x, y)) = a.explode() {
                b.add_left(y);
                Some((x, 0))
            } else if let Some((x, y)) = b.explode() {
                a.add_right(x);
                Some((0, y))
            } else {
                None
            }
        } else {
            None
        }
    }
    fn split(&mut self) -> bool {
        if let Self::Regular { n, lv } = self {
            if *n >= 10 {
                let x = *n / 2;
                let y = *n - x;
                let a = Self::Regular { lv: *lv + 1, n: x }.into();
                let b = Self::Regular { lv: *lv + 1, n: y }.into();
                *self = Self::Pair { lv: *lv, a, b };
                true
            } else {
                false
            }
        } else {
            let (a, b) = self.as_pair().unwrap();
            a.split() || b.split()
        }
    }
}
fn main() {
    let lines = read_lines("./input18.txt");
    let numbers = lines
        .map(|l| SnailFish::parse(0, l.as_str()).0)
        .collect::<Vec<_>>();
    let mut n = numbers.clone().into_iter().reduce(SnailFish::add).unwrap();
    n.reduce();
    println!("{}", n.magnitude());

    let max = numbers
        .into_iter()
        .tuple_combinations()
        .map(|(i, j)| i.add(j).magnitude())
        .max();
    println!("{:?}", max);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explode() {
        fn test(a: &str, b: &str) {
            let mut n = SnailFish::parse(0, a).0;
            n.explode();
            let t = SnailFish::parse(0, b).0;
            assert_eq!(n, t);
        }
        test("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        test("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        test("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        test(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        );
        test(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );
    }

    #[test]
    fn test_split() {
        fn test(a: &str, b: &str) {
            let mut n = SnailFish::parse(0, a).0;
            n.split();
            let t = SnailFish::parse(0, b).0;
            assert_eq!(n, t);
        }
        test("10", "[5,5]");
        test("11", "[5,6]");
        test("12", "[6,6]");
        test(
            "[[[[0,7],4],[15,[0,13]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
        );
        test(
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
        );
    }
}
