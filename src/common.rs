use std::io::BufRead;
use std::path::Path;
use std::{fs::File, io::BufReader};

pub fn read_lines<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("open file");
    BufReader::new(file).lines().map(|lines| lines.unwrap())
}

pub fn read_u8_matrix(filename: &str) -> Vec<Vec<u8>> {
    read_lines(filename)
        .into_iter()
        .map(|s| s.chars().map(|c| c as u8 - 48).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub struct ReadChunks {
    buf: Vec<String>,
    lines: Box<dyn Iterator<Item = String>>,
}

impl ReadChunks {
    pub fn new<P: 'static>(filename: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            buf: vec![],
            lines: Box::new(read_lines(filename)),
        }
    }
}

impl Iterator for ReadChunks {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        for line in &mut self.lines {
            if line.is_empty() {
                break;
            }
            self.buf.push(line);
        }
        if self.buf.is_empty() {
            None
        } else {
            Some(std::mem::take(&mut self.buf))
        }
    }
}

pub fn adj_list(pos: (usize, usize), max: (usize, usize)) -> Vec<(usize, usize)> {
    let mut adj = vec![];
    if pos.0 > 0 {
        adj.push((pos.0 - 1, pos.1));
    }
    if pos.0 < max.0 - 1 {
        adj.push((pos.0 + 1, pos.1));
    }
    if pos.1 > 0 {
        adj.push((pos.0, pos.1 - 1));
    }
    if pos.1 < max.1 - 1 {
        adj.push((pos.0, pos.1 + 1));
    }
    adj
}

pub fn find_edge(points: impl IntoIterator<Item = (i32, i32)>) -> (i32, i32, i32, i32) {
    points
        .into_iter()
        .fold((i32::MAX, i32::MIN, i32::MAX, i32::MIN), |acc, x| {
            (
                std::cmp::min(acc.0, x.0),
                std::cmp::max(acc.1, x.0),
                std::cmp::min(acc.2, x.1),
                std::cmp::max(acc.3, x.1),
            )
        })
}
