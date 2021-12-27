use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};

use advent_of_code::common::{read_lines, shortest_path};
use itertools::Itertools;

type Pos = (usize, usize);
struct Map {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
    points: HashMap<char, Pos>,
    // keys: Vec<char>,
}

impl Map {
    fn new(mut map: Vec<Vec<char>>) -> Self {
        let (width, height) = (map[0].len(), map.len());
        let mut points = HashMap::new();
        for (y, row) in map.iter_mut().enumerate() {
            for (x, ch) in row.iter_mut().enumerate() {
                if *ch == '.' || *ch == '#' {
                    continue;
                }
                if *ch == '@' {
                    *ch = '0';
                }
                points.insert(*ch, (x, y));
            }
        }
        Self {
            map,
            width,
            height,
            points,
            // keys: vec!['@'],
        }
    }

    fn get(&self, (x, y): Pos) -> char {
        self.map[y][x]
    }

    fn neighbors(&self, (x, y): Pos) -> Vec<Pos> {
        let mut neighbors = vec![];
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < self.width - 1 {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < self.height - 1 {
            neighbors.push((x, y + 1));
        }
        neighbors
    }

    fn can_move(&self, p: Pos) -> bool {
        self.get(p) != '#'
    }

    fn all_moves(&self, p: Pos, keys: &[char]) -> Vec<(Pos, usize)> {
        let ch = self.get(p);
        // void (numbers|'.') || door with key || key we had already
        if ch.is_numeric() || ch == '.' || keys.contains(&ch.to_ascii_lowercase()) {
            self.neighbors(p)
                .into_iter()
                .filter_map(|p| if self.can_move(p) { Some((p, 1)) } else { None })
                .collect()
        } else {
            vec![]
        }
    }

    fn all_keys(&self) -> Vec<char> {
        self.points
            .keys()
            .filter(|ch| ch.is_ascii_lowercase())
            .cloned()
            .sorted()
            .collect_vec()
    }
}

fn solve1(map: &Map) -> usize {
    use std::hash::Hash;
    #[derive(Debug, Eq, Clone)]
    struct Node {
        pos: char,
        keys: Vec<char>,
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            self.keys.cmp(&other.keys)
        }
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.keys.partial_cmp(&other.keys)
        }
    }

    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            (self.pos == '?' || other.pos == '?' || self.pos == other.pos)
                && self.keys == other.keys
        }
    }

    impl Hash for Node {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.keys.hash(state);
        }
    }

    let start = Node {
        pos: '0',
        keys: vec![],
    };
    let end = Node {
        pos: '?',
        keys: map.all_keys(),
    };
    fn edges(map: &Map, x: &Node) -> Vec<(Node, usize)> {
        println!("edges: {:?}", x);
        let keys = x.keys.clone();
        let start = map.points[&x.pos];
        let all_keys = map
            .points
            .iter()
            .filter_map(|(ch, p)| {
                if ch.is_ascii_lowercase() && !keys.contains(ch) {
                    Some(*p)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if all_keys.is_empty() {
            return vec![];
        }
        shortest_path_multi(start, &all_keys, |&x| map.all_moves(x, &keys))
            .into_iter()
            .filter(|x| x.1.is_some())
            .map(|(p, cost)| {
                let mut n = Node {
                    pos: map.get(p),
                    keys: keys.clone(),
                };
                n.keys.push(map.get(p));
                n.keys.sort_unstable();
                (n, cost.unwrap())
            })
            .collect_vec()
    }

    shortest_path(start, end, |x| edges(map, x)).unwrap()
}

fn main() {
    let map = read_lines("input18.txt")
        .map(|line| line.chars().collect())
        .collect();
    let map = Map::new(map);
    println!("{}", solve1(&map));
}

pub fn shortest_path_multi<T, F>(start: T, end: &[T], edges: F) -> HashMap<T, Option<usize>>
where
    T: std::hash::Hash + Eq + Clone + Ord,
    F: Fn(&T) -> Vec<(T, usize)>,
{
    let mut costs: HashMap<T, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut ret = end
        .iter()
        .map(|x| (x.clone(), None))
        .collect::<HashMap<_, _>>();
    costs.entry(start.clone()).or_insert(0);
    heap.push(Reverse((0, start)));

    while let Some(Reverse((cost, pos))) = heap.pop() {
        if let Some(c) = ret.get_mut(&pos) {
            *c = Some(cost);
        }
        if ret.values().all(Option::is_some) {
            break;
        }
        if cost > costs[&pos] {
            continue;
        }
        for (new_node, new_cost) in edges(&pos) {
            let new_cost = cost + new_cost;
            let d = costs.entry(new_node.clone()).or_insert(usize::MAX);
            if new_cost < *d {
                heap.push(Reverse((new_cost, new_node)));
                *d = new_cost;
            }
        }
    }
    ret
}
