use std::io;

#[derive(Debug)]
struct Board {
    numbers: Vec<Vec<i32>>,
    called: Vec<Vec<bool>>,
    just_called: i32,
}

impl Board {
    fn parse(stream: &mut io::Stdin) -> Option<Board> {
        let mut buf = String::new();
        let mut lines: Vec<Vec<i32>> = Vec::new();
        loop {
            let n = stream.read_line(&mut buf).expect("Failed to read stdin");
            if n == 0 || buf.trim().len() == 0 {
                break;
            }
            let mut line: Vec<i32> = Vec::new();
            for part in buf.trim().split(" ") {
                if part.trim().len() > 0 {
                    let val: i32 = part.trim().parse().expect("Invalid integer");
                    line.push(val);
                }
            }
            lines.push(line);
            buf.clear();
        }
        let called = lines
            .iter()
            .map(|line| line.iter().map(|_| false).collect())
            .collect();
        if lines.len() == 0 {
            return None;
        }
        Some(Board {
            numbers: lines,
            called,
            just_called: 0,
        })
    }

    fn call(&mut self, number: i32) {
        for i in 0..self.numbers.len() {
            for j in 0..self.numbers[i].len() {
                if self.numbers[i][j] == number {
                    self.called[i][j] = true;
                    self.just_called = number
                }
            }
        }
    }

    fn clear(&mut self) {
        for i in 0..self.numbers.len() {
            for j in 0..self.numbers[i].len() {
                self.called[i][j] = false;
            }
        }
    }

    fn score(&self) -> i64 {
        let mut score: i64 = 0;
        for i in 0..self.numbers.len() {
            for j in 0..self.numbers[i].len() {
                if !self.called[i][j] {
                    let num: i64 = self.numbers[i][j].into();
                    score += num;
                }
            }
        }
        let num: i64 = self.just_called.into();
        score * num
    }

    fn is_winner(&self) -> bool {
        for i in 0..self.numbers.len() {
            let mut row_wins = true;
            for j in 0..self.numbers[i].len() {
                if !self.called[i][j] {
                    row_wins = false;
                    break;
                }
            }
            if row_wins {
                // println!("Row {} in {:?} wins", i, self);
                return true;
            }
        }

        for j in 0..self.numbers[0].len() {
            let mut col_wins = true;
            for i in 0..self.numbers.len() {
                if !self.called[i][j] {
                    col_wins = false;
                    break;
                }
            }
            if col_wins {
                // println!("Col {} in {:?} wins", j, self);
                return true;
            }
        }

        return false;
    }
}

fn parse_input() -> (Vec<i32>, Vec<Board>) {
    let mut buffer = String::new();
    let mut state = 0;
    let mut calls: Vec<i32> = Vec::new();
    let mut boards: Vec<Board> = Vec::new();

    loop {
        if state == 2 {
            match Board::parse(&mut io::stdin()) {
                Some(board) => boards.push(board),
                None => break,
            }
        } else {
            let n = io::stdin()
                .read_line(&mut buffer)
                .expect("Failed to read stdin");
            if n == 0 {
                // End of input
                break;
            }
            match state {
                0 => {
                    calls = buffer
                        .trim()
                        .split(",")
                        .map(|x| {
                            let val: i32 = x.parse().expect("Invalid integer");
                            val
                        })
                        .collect();
                    state = 1;
                }
                1 => {
                    state = 2;
                }
                _ => panic!("Invalid state {}", state),
            };
            buffer.clear();
        }
    }
    (calls, boards)
}

fn part1(calls: &[i32], boards: &mut [Board]) -> i64 {
    for &call in calls {
        for board in boards.iter_mut() {
            board.call(call);
            if board.is_winner() {
                return board.score();
            }
        }
    }
    panic!("No winner");
}

fn part2(calls: &[i32], boards: &mut [Board]) -> i64 {
    let mut winners: usize = 0;
    let n = boards.len();
    for &call in calls {
        for board in boards.iter_mut() {
            if !board.is_winner() {
                board.call(call);
                if board.is_winner() {
                    winners += 1;
                    if winners == n {
                        return board.score();
                    }
                }
            }
        }
    }
    panic!("Not all boards win");
}

fn main() {
    let (calls, mut boards) = parse_input();
    println!("Part 1: {}", part1(&calls[..], boards.as_mut_slice()));
    for board in boards.iter_mut() {
        board.clear();
    }
    println!("Part 2: {}", part2(&calls[..], boards.as_mut_slice()));
}
