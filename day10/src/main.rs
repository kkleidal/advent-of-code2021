use std::collections::LinkedList;
use std::io;

#[derive(Debug, PartialEq, Clone)]
enum BraceKind {
    SQUARE,
    PAREN,
    CURLY,
    ANGLE,
}

#[derive(Debug)]
struct Brace {
    kind: BraceKind,
    open: bool,
}

impl BraceKind {
    fn score(&self) -> i64 {
        match self {
            BraceKind::SQUARE => 57,
            BraceKind::PAREN => 3,
            BraceKind::CURLY => 1197,
            BraceKind::ANGLE => 25137,
        }
    }

    fn autoscore(&self) -> i64 {
        match self {
            BraceKind::SQUARE => 2,
            BraceKind::PAREN => 1,
            BraceKind::CURLY => 3,
            BraceKind::ANGLE => 4,
        }
    }
}

fn parse() -> Vec<Vec<Brace>> {
    let mut buffer = String::new();
    let mut out: Vec<Vec<Brace>> = Vec::new();
    loop {
        let n = io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read stdin");
        if n == 0 {
            // End of input
            break;
        }
        let row: Vec<Brace> = buffer
            .trim()
            .chars()
            .map(|c| match c {
                '[' => Brace {
                    kind: BraceKind::SQUARE,
                    open: true,
                },
                ']' => Brace {
                    kind: BraceKind::SQUARE,
                    open: false,
                },
                '(' => Brace {
                    kind: BraceKind::PAREN,
                    open: true,
                },
                ')' => Brace {
                    kind: BraceKind::PAREN,
                    open: false,
                },
                '{' => Brace {
                    kind: BraceKind::CURLY,
                    open: true,
                },
                '}' => Brace {
                    kind: BraceKind::CURLY,
                    open: false,
                },
                '<' => Brace {
                    kind: BraceKind::ANGLE,
                    open: true,
                },
                '>' => Brace {
                    kind: BraceKind::ANGLE,
                    open: false,
                },
                _ => panic!("Invalid character {:?}", c),
            })
            .collect();
        out.push(row);
        buffer.clear();
    }
    out
}

fn part1(rows: &[Vec<Brace>]) -> i64 {
    let mut score = 0;
    for row in rows.iter() {
        let mut stack: LinkedList<BraceKind> = LinkedList::new();
        for brace in row.iter() {
            if brace.open {
                stack.push_back(brace.kind.clone());
            } else {
                if stack.len() == 0 || stack.pop_back().expect("Must not be empty") != brace.kind {
                    // Invalid
                    score += brace.kind.score();
                    break;
                }
            }
        }
    }
    score
}

fn part2(rows: &[Vec<Brace>]) -> i64 {
    let mut all_scores: Vec<i64> = Vec::new();
    for row in rows.iter() {
        let mut stack: LinkedList<BraceKind> = LinkedList::new();
        let mut valid = true;
        for brace in row.iter() {
            if brace.open {
                stack.push_front(brace.kind.clone());
            } else {
                if stack.len() == 0 || stack.pop_front().expect("Must not be empty") != brace.kind {
                    // Invalid
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            let mut score = 0;
            loop {
                match stack.pop_front() {
                    Some(kind) => {
                        score = score * 5 + kind.autoscore();
                    }
                    None => break, // stack Empty
                }
            }
            all_scores.push(score);
        }
    }
    all_scores.sort();
    all_scores[all_scores.len() / 2]
}

fn main() {
    let rows = parse();
    println!("Rows: {:?}", rows);
    println!("Part 1: {}", part1(rows.as_slice()));
    println!("Part 2: {}", part2(rows.as_slice()));
}
