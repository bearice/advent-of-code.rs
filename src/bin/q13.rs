use std::cmp::Ordering;

use advent_of_code::common::ReadChunks;
use json::JsonValue;

#[derive(Debug, PartialEq, Eq)]
enum Expr {
    List(Vec<Expr>),
    Value(i32),
}

impl Expr {
    fn parse(input: &str) -> Expr {
        json::parse(input).unwrap().into()
    }
    fn is_value(&self) -> bool {
        matches!(self, Expr::Value(_))
    }
    fn as_value(&self) -> i32 {
        if let Expr::Value(v) = self {
            *v
        } else {
            unreachable!()
        }
    }
    fn as_list(&self) -> Vec<&Expr> {
        match self {
            Self::List(l) => l.iter().collect(),
            Self::Value(_) => vec![self],
        }
    }
}

impl Ord for Expr {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_value() && other.is_value() {
            self.as_value().cmp(&other.as_value())
        } else {
            let left = self.as_list();
            let right = other.as_list();
            let len = (left.len(), right.len());
            for (l, r) in left.into_iter().zip(right) {
                let ret = l.cmp(r);
                if ret == Ordering::Equal {
                    continue;
                } else {
                    return ret;
                }
            }
            len.0.cmp(&len.1)
        }
    }
}

impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Expr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<JsonValue> for Expr {
    fn from(input: JsonValue) -> Self {
        match input {
            JsonValue::Number(n) => Self::Value(f32::from(n) as i32),
            JsonValue::Array(v) => Self::List(v.into_iter().map(Into::into).collect()),
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = ReadChunks::new("input13.txt");
    let mut sum = 0;
    let k1 = Expr::parse("[[2]]");
    let k2 = Expr::parse("[[6]]");
    let mut all = vec![k1, k2];
    for (lines, idx) in input.zip(1..) {
        let left = Expr::parse(&lines[0]);
        let right = Expr::parse(&lines[1]);
        let cmp = left.cmp(&right);
        if cmp == Ordering::Less {
            sum += idx;
        }
        all.push(left);
        all.push(right);
    }
    println!("{}", sum);
    all.sort();
    let key1 = all.binary_search(&Expr::parse("[[2]]")).unwrap() + 1;
    let key2 = all.binary_search(&Expr::parse("[[6]]")).unwrap() + 1;
    println!("{}", key1 * key2);
}
