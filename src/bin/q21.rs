struct Dice {
    n: usize,
    cnt: usize,
}
impl Dice {
    fn new() -> Self {
        Dice { n: 0, cnt: 0 }
    }
    fn roll(&mut self) -> usize {
        let ret = self.n + 1;
        self.cnt += 1;
        self.n += 1;
        self.n %= 100;
        ret
    }
    fn roll3(&mut self) -> usize {
        self.roll() + self.roll() + self.roll()
    }
}

#[derive(Debug, Clone, Copy)]
struct Player {
    pos: usize,
    score: usize,
}
impl Player {
    fn new(pos: usize) -> Self {
        Player {
            pos: pos - 1,
            score: 0,
        }
    }
    fn play(&self, n: usize) -> Self {
        let pos = (self.pos + n) % 10;
        let score = self.score + pos + 1;
        Player { pos, score }
    }
}

fn q1(p1: Player, p2: Player) {
    let mut players = [p1, p2];
    let mut dice = Dice::new();

    while players.iter().all(|p| p.score < 1000) {
        for p in &mut players {
            *p = p.play(dice.roll3());
        }
    }
    let loser = players.iter().min_by_key(|p| p.score).unwrap();
    println!("{}", loser.score * dice.cnt);
}

fn q2(p1: Player, p2: Player) {
    let mut all_worlds = vec![(1usize, p1, p2)];
    let mut won = Vec::new();
    let dice_counts = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    while !all_worlds.is_empty() {
        let mut new_worlds = Vec::new();
        for (w, p1, p2) in &all_worlds {
            for &(n, worlds) in &dice_counts {
                let p1 = p1.play(n);
                let w = w * worlds;
                if p1.score >= 21 {
                    won.push((0, w));
                } else {
                    for &(n2, worlds2) in &dice_counts {
                        let w = w * worlds2;
                        let p2 = p2.play(n2);
                        if p2.score >= 21 {
                            won.push((1, w));
                        } else {
                            new_worlds.push((w, p1, p2));
                        }
                    }
                }
            }
        }
        all_worlds = new_worlds;
    }
    let mut worlds = [0, 0];
    for (id, w) in won {
        worlds[id] += w;
    }
    println!("{:?}", worlds);
}

fn main() {
    let p1 = Player::new(5);
    let p2 = Player::new(6);
    q1(p1, p2);
    q2(p1, p2);
}
