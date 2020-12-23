use std::io::{BufRead, Lines, Result};
use std::path::Path;
use std::{fs::File, io::BufReader};

pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub struct ReadChunks {
    buf: Vec<String>,
    lines: Lines<BufReader<File>>,
}

impl ReadChunks {
    pub fn new<P>(filename: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            buf: vec![],
            lines: read_lines(filename).unwrap(),
        }
    }
}

impl Iterator for ReadChunks {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(Ok(line)) = self.lines.next() {
            if line.len() == 0 {
                break;
            }
            self.buf.push(line);
        }
        Some(std::mem::replace(&mut self.buf, Vec::new()))
    }
}
