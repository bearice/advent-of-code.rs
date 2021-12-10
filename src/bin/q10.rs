use advent_of_code::common::read_lines;

fn main() {
    let lines = read_lines("input10.txt");
    let (s1, s2): (Vec<usize>, Vec<usize>) = lines.map(score_of_line).unzip();
    let mut s2 = s2.into_iter().filter(|x| *x > 0).collect::<Vec<_>>();
    s2.sort_unstable();
    println!("{} {}", s1.into_iter().sum::<usize>(), s2[s2.len() / 2]);
}

fn score_of_line(line: String) -> (usize, usize) {
    let mut stack = vec![];
    for ch in line.chars() {
        match ch {
            '(' | '[' | '<' | '{' => stack.push(ch),
            _ => {
                let top = stack.pop().unwrap();
                let x = score_of1(ch);
                if top != x.0 {
                    return (x.1, 0);
                }
            }
        }
    }
    let mut score2 = 0;
    while !stack.is_empty() {
        score2 = score2 * 5 + score_of2(stack.pop().unwrap());
    }
    (0, score2)
}

fn score_of1(ch: char) -> (char, usize) {
    match ch {
        ')' => ('(', 3),
        ']' => ('[', 57),
        '}' => ('{', 1197),
        '>' => ('<', 25137),
        _ => panic!("not possible"),
    }
}

fn score_of2(ch: char) -> usize {
    match ch {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("not possible: {}", ch),
    }
}
