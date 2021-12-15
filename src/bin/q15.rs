use std::{cmp::Reverse, collections::BinaryHeap};

use advent_of_code::common::read_u8_matrix;

fn main() {
    let map = read_u8_matrix("input15.txt");
    println!(
        "{:?}",
        shortest_path(&map, (0, 0), (map.len() - 1, map[0].len() - 1))
    );
    // println!("{:?}", enlarge(vec![vec![8]]))
    let map = enlarge(map);
    println!(
        "{:?}",
        shortest_path(&map, (0, 0), (map.len() - 1, map[0].len() - 1))
    );
}

fn enlarge(map: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut ret = Vec::new();
    for x in 0..5 {
        for line in &map {
            let mut new_line = Vec::new();
            for y in 0..5 {
                for &n in line {
                    let n = n as u32 + x + y - 1;
                    new_line.push((n % 9) as u8 + 1);
                }
            }
            ret.push(new_line);
        }
    }
    ret
}

// Dijkstra’s algorithm from rust std library documentation
fn shortest_path(nodes: &[Vec<u8>], start: (usize, usize), goal: (usize, usize)) -> Option<usize> {
    let max = (nodes.len(), nodes[0].len());
    let mut dist: Vec<Vec<usize>> = (0..nodes.len())
        .map(|_| (0..nodes[0].len()).map(|_| usize::MAX).collect())
        .collect();

    let mut heap = BinaryHeap::new();

    dist[start.0][start.1] = 0;
    heap.push(Reverse((0, start)));

    while let Some(Reverse((cost, pos))) = heap.pop() {
        if pos == goal {
            return Some(cost);
        }
        if cost > dist[pos.0][pos.1] {
            continue;
        }
        for edge in adj_list(pos, max) {
            let next = (cost + nodes[edge.0][edge.1] as usize, edge);
            if next.0 < dist[edge.0][edge.1] {
                heap.push(Reverse(next));
                dist[edge.0][edge.1] = next.0;
            }
        }
    }
    None
}

fn adj_list(pos: (usize, usize), max: (usize, usize)) -> Vec<(usize, usize)> {
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
