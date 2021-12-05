use advent_of_code::common::ReadChunks;

fn main() {
    let mut chunks = ReadChunks::new("./input4.txt");
    let cards = chunks.next().unwrap();
    let cards = cards[0]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u8>>();
    let mut boards = chunks
        .map(|lines| {
            (
                lines
                    .into_iter()
                    .map(|s| {
                        s.split_ascii_whitespace()
                            .map(|v| (v.parse().unwrap(), false))
                            .collect::<Vec<(u8, bool)>>()
                    })
                    .collect::<Vec<_>>(),
                false,
            )
        })
        .collect::<Vec<_>>();
    let mut score = vec![];
    for n in cards {
        for (board, won) in boards.iter_mut() {
            if *won {
                continue;
            }
            'next: for x in 0..5 {
                for y in 0..5 {
                    if board[x][y].0 == n {
                        board[x][y].1 = true;
                        if check_board(board, x, y) {
                            score.push(score_board(board, n));
                            *won = true;
                        }
                        break 'next;
                    }
                }
            }
        }
    }
    println!("first: {} last: {}", score[0], score[score.len() - 1]);
}

fn check_board(board: &[Vec<(u8, bool)>], x: usize, y: usize) -> bool {
    board[x].iter().all(|(_, r)| *r) || board.iter().map(|b| b[y]).all(|(_, r)| r)
}

fn score_board(board: &[Vec<(u8, bool)>], n: u8) -> u32 {
    let mut ret = 0;
    for x in 0..5 {
        for y in 0..5 {
            if !board[x][y].1 {
                ret += board[x][y].0 as u32;
            }
        }
    }
    ret * n as u32
}
