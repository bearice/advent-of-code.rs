use advent_of_code::common::read_lines;
fn main() {
    let lines = read_lines("./input2.txt")
        .map(|s| {
            let (x, y) = s.split_once(' ').unwrap();
            (x.chars().next().unwrap(), y.chars().next().unwrap())
        })
        .collect::<Vec<_>>();
    let total1: i32 = lines.iter().map(|x| score1(*x)).sum();
    println!("{}", total1);
    let total2: i32 = lines.iter().map(|x| score2(*x)).sum();
    println!("{}", total2);
}
// opponent : A for Rock, B for Paper, and C for Scissors
// mine: X for Rock, Y for Paper, and Z for Scissors
// The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
// plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won)
fn score1((opponent, mine): (char, char)) -> i32 {
    let base_score = match mine {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    };
    let outcome_score = match (opponent, mine) {
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
        _ => 0,
    };
    base_score + outcome_score
}

// opponent : A for Rock, B for Paper, and C for Scissors
// outcome: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
// The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
// plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won)
fn score2((opponent, outcome): (char, char)) -> i32 {
    let outcome_score = match outcome {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => panic!(),
    };
    let base_score = match (opponent, outcome) {
        ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 1,
        ('A', 'Z') | ('B', 'Y') | ('C', 'X') => 2,
        ('A', 'X') | ('B', 'Z') | ('C', 'Y') => 3,
        _ => panic!(),
    };
    base_score + outcome_score
}
