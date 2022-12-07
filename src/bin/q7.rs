use advent_of_code::common::read_lines;
use itertools::Itertools;

#[allow(dead_code)]
#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    size: usize,
    is_dir: bool,
    children: Vec<Node<'a>>,
}

impl<'a> Node<'a> {
    fn new(name: &'a str, size: usize, is_dir: bool) -> Node<'a> {
        Node {
            name,
            size,
            is_dir,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: Node<'a>) {
        self.size += child.size;
        self.children.push(child);
    }

    fn parse<T: Iterator<Item = &'a String>>(name: &'a str, input: &mut T) -> Node<'a> {
        let mut current_dir = Node::new(name, 0, true);
        while let Some(line) = input.next() {
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "$" => {
                    if parts[1] == "ls" {
                        continue;
                    }
                    assert_eq!(parts[1], "cd");
                    let name = parts[2];
                    if name == ".." {
                        return current_dir;
                    }
                    current_dir.add_child(Node::parse(name, input));
                }
                "dir" => {}
                size => {
                    let size = size.parse().unwrap();
                    current_dir.add_child(Node::new(parts[1], size, false));
                }
            }
        }
        current_dir
    }

    fn walk<F, T>(&self, initial: T, f: &F) -> T
    where
        F: Fn(&Node<'a>, T) -> T,
    {
        let mut ret = f(self, initial);
        for node in &self.children {
            ret = node.walk(ret, f);
        }
        ret
    }
}

fn main() {
    let input = read_lines("input7.txt").collect_vec();
    let root = Node::parse("/", &mut input.iter().skip(1));
    println!(
        "{}",
        root.walk(0, &|node, acc| {
            if node.is_dir && node.size < 100000 {
                acc + node.size
            } else {
                acc
            }
        })
    );

    let total = 70000000;
    let free = total - root.size;
    let required = 30000000;
    println!(
        "{}",
        root.walk(total, &|node, acc| {
            if node.is_dir && free + node.size > required {
                acc.min(node.size)
            } else {
                acc
            }
        })
    );
}
