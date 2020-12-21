use std::collections::HashMap;

#[derive(Debug)]
struct Day15 {
    numbers: HashMap<usize, (usize, usize)>,
    last: usize,
    init: Vec<usize>,
    n: usize,
}

impl Day15 {
    fn new(init: Vec<usize>) -> Self {
        Self {
            numbers: HashMap::new(),
            last: 0,
            n: 0,
            init,
        }
    }
}

impl Iterator for Day15 {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n + 1;
        self.last = if self.n < self.init.len() {
            self.init[self.n]
        } else {
            if let Some(x) = &self.numbers.get(&self.last) {
                x.1 - x.0
            } else {
                0
            }
        };
        self.n = n;
        if let Some(x) = self.numbers.get_mut(&self.last) {
            x.0 = x.1;
            x.1 = n;
        // println!("{} {:?}", n, x)
        } else {
            self.numbers.insert(self.last, (n, n));
        }
        Some(self.last)
    }
}
fn main() {
    let i = vec![0, 13, 1, 16, 6, 17];
    let mut it = Day15::new(i);
    println!("{}", it.nth(30000000 - 1).unwrap());
}
