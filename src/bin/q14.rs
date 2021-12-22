use std::collections::HashMap;

use advent_of_code::common::read_lines;

struct Rule {
    from: Vec<(String, usize)>,
    to: (String, usize),
}

impl Rule {
    fn new(from: Vec<(String, usize)>, to: (String, usize)) -> Self {
        Self { from, to }
    }
    fn from_line(line: String) -> Self {
        fn parse_pair(pair: &str) -> (String, usize) {
            let (count, name) = pair.split_once(' ').unwrap();
            (name.to_string(), count.parse::<usize>().unwrap())
        }
        let (from, to) = line.split_once(" => ").unwrap();
        let to = parse_pair(to);
        let from = from.split(", ").map(parse_pair).collect::<Vec<_>>();
        Self::new(from, to)
    }
    fn get_inputs(&self, n: usize) -> (usize, Vec<(String, usize)>) {
        let make_cnt = (n as f64 / self.to.1 as f64).ceil() as usize;
        let remain = make_cnt * self.to.1 - n;
        (
            remain,
            self.from
                .iter()
                .map(|(name, count)| (name.to_owned(), make_cnt * count))
                .collect(),
        )
    }
}

fn fuel_to_ore(rules: &HashMap<String, Rule>, n: usize) -> usize {
    let mut needs = vec![("FUEL".to_owned(), n)];
    let mut ore_count = 0;
    let mut remains = HashMap::new();
    while let Some((name, count)) = needs.pop() {
        if name == "ORE" {
            ore_count += count;
        } else {
            let remain = remains.entry(name.clone()).or_insert(0);
            if *remain >= count {
                *remain -= count;
            } else {
                let to_make = count - *remain;
                let rule = &rules[&name];
                let (leftover, inputs) = rule.get_inputs(to_make);
                *remain = leftover;
                needs.extend(inputs)
            }
        }
    }
    ore_count
}
fn main() {
    let rules = read_lines("input14.txt")
        .map(Rule::from_line)
        .map(|rule| (rule.to.0.to_owned(), rule))
        .collect::<HashMap<_, _>>();

    let one_fuel = fuel_to_ore(&rules, 1);
    println!("{}", one_fuel);

    let total_ore = 1000000000000;
    let mut fuel_min = total_ore / one_fuel;
    let mut fuel_max = fuel_min * 2;
    while fuel_max - fuel_min > 1 {
        let fuel = (fuel_min + fuel_max) / 2;
        let cost = fuel_to_ore(&rules, fuel);
        if cost > total_ore {
            fuel_max = fuel;
        } else {
            fuel_min = fuel;
        }
    }

    println!("{}", fuel_min);
}
