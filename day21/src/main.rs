use std::collections::HashMap;
use std::hash::{Hash, Hasher};

fn part1(player1_start: u64, player2_start: u64) -> u64 {
    let mut positions = vec![player1_start, player2_start];
    let mut scores: Vec<u64> = vec![0, 0];
    let mut die: u64 = 1;
    let mut turn: usize = 0;
    let mut rolls: u64 = 0;
    while !scores.iter().any(|score| *score >= 1000) {
        let move_positions: u64 = (0..3)
            .map(|_| {
                let value = die;
                die = (die % 100) + 1;
                rolls += 1;
                value
            })
            .sum();
        let my_pos = positions[turn];
        let new_pos = ((my_pos + move_positions - 1) % 10) + 1;
        positions[turn] = new_pos;
        scores[turn] += new_pos;
        turn = (turn + 1) % positions.len();
    }
    scores.iter().min().unwrap() * rolls
}

#[derive(Debug, Clone)]
struct GameState {
    turn: usize,
    scores: Vec<u64>,
    positions: Vec<u64>,
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.turn == other.turn
            && self.scores.len() == other.scores.len()
            && self
                .scores
                .iter()
                .zip(other.scores.iter())
                .all(|(x, y)| x == y)
            && self.positions.len() == other.positions.len()
            && self
                .positions
                .iter()
                .zip(other.positions.iter())
                .all(|(x, y)| x == y)
    }
}

impl Eq for GameState {}

impl Hash for GameState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.turn.hash(state);
        self.scores.len().hash(state);
        self.positions.len().hash(state);
        for score in self.scores.iter() {
            score.hash(state);
        }
        for position in self.positions.iter() {
            position.hash(state);
        }
    }
}

impl GameState {
    fn initial(player1_start: u64, player2_start: u64) -> Self {
        Self {
            turn: 0,
            scores: vec![0, 0],
            positions: vec![player1_start, player2_start],
        }
    }

    fn finished(&self) -> bool {
        self.scores.iter().any(|x| *x >= 21)
    }

    fn round(&self) -> Vec<Self> {
        if self.finished() {
            return vec![self.clone()];
        }
        let turn = self.turn;
        let mut out: Vec<Self> = Vec::new();
        for roll1 in 1..4 {
            for roll2 in 1..4 {
                for roll3 in 1..4 {
                    let mut positions = self.positions.clone();
                    let mut scores = self.scores.clone();

                    let move_positions = roll1 + roll2 + roll3;
                    let my_pos = positions[turn];
                    let new_pos = ((my_pos + move_positions - 1) % 10) + 1;
                    positions[turn] = new_pos;
                    scores[turn] += new_pos;

                    let new_turn = (turn + 1) % positions.len();

                    out.push(Self {
                        turn: new_turn,
                        scores,
                        positions,
                    });
                }
            }
        }
        out
    }
}

fn part2(player1_start: u64, player2_start: u64) -> usize {
    let mut states: HashMap<GameState, usize> = HashMap::new();
    states.insert(GameState::initial(player1_start, player2_start), 1);
    while !states.iter().all(|(k, _)| k.finished()) {
        println!(
            "{} / {} games underway",
            states
                .iter()
                .map(|(k, v)| if k.finished() { 0 } else { *v })
                .sum::<usize>(),
            states.iter().map(|(_, v)| *v).sum::<usize>(),
        );
        let mut new_states: HashMap<GameState, usize> = HashMap::new();
        for (state, count) in states.iter() {
            for new_state in state.round() {
                *new_states.entry(new_state).or_insert(0) += count;
            }
        }
        states = new_states
    }
    (0..2)
        .map(|player| {
            states
                .iter()
                .map(|(state, games)| {
                    if state.scores[player] > state.scores[1 - player] {
                        *games
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .max()
        .unwrap()
}

fn main() {
    println!("Part 1: Example: {}", part1(4, 8));
    println!("Part 1: Mine: {}", part1(1, 2));
    println!("Part 2: Example: {}", part2(4, 8));
    println!("Part 2: Mine: {}", part2(1, 2));
}
