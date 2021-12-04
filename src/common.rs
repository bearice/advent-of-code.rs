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
