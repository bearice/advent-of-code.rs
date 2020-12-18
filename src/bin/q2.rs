macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}
#[derive(Debug)]
struct Rule {
    ch: char,
    min: usize,
    max: usize,
}
impl Rule {
    #[allow(unused)]
    fn check(self: &Self, word: &str) -> bool {
        let mut cnt = 0;
        for ch in word.chars() {
            if ch == self.ch {
                cnt = cnt + 1;
            }
        }
        cnt >= self.min && cnt <= self.max
    }
    fn check2(self: &Self, word: &str) -> bool {
        let slice = word.get(self.min..self.max + 1).unwrap();
        let b = slice.starts_with(self.ch) ^ slice.ends_with(self.ch);
        // println!("{:?} {} {}", self, word, slice);
        // println!(
        //     "{} {} {}",
        //     slice.starts_with(self.ch),
        //     slice.ends_with(self.ch),
        //     b
        // );
        slice.len() > 1 && b
    }
}
impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        let (min, max, ch) = scan!(s, |x| x == ' ' || x == '-', usize, usize, char);
        Rule {
            min: min.unwrap(),
            max: max.unwrap(),
            ch: ch.unwrap(),
        }
    }
}

fn validate(s: &String) -> bool {
    let mut s = s.split(':');
    let r = Rule::from(s.next().unwrap());
    let w = s.next().unwrap();
    r.check2(w)
}
use advent_of_code::common::read_lines;
fn main() {
    let mut lines = read_lines("./input2.txt").unwrap();
    // Consumes the iterator, returns an (Optional) String
    let mut i = 0u32;
    while let Some(Ok(line)) = lines.next() {
        if validate(&line) {
            i = i + 1;
            // } else {
            //     println!("{}", line);
        }
    }
    println!("{}", i);
}
