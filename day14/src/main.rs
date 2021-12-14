#![feature(linked_list_cursors)]

use std::collections::LinkedList;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct PolymerTemplate {
    seq: LinkedList<char>,
    rule_tree: HashMap<char, HashMap<char, char>>,  // map first to second to middle
}

impl PolymerTemplate {
    fn parse() -> PolymerTemplate {
        let mut seq: LinkedList<char> = LinkedList::new();
        let mut rule_tree: HashMap<char, HashMap<char, char>> = HashMap::new();
        let mut state = 0;
        let mut buffer = String::new();
        loop {
            let n = io::stdin()
                .read_line(&mut buffer)
                .expect("Failed to read stdin");
            if n == 0 {
                // End of input
                break;
            }
            match state {
                0 => {
                    if buffer.trim().len() == 0 {
                        state = 1;
                        continue;
                    }
                    for c in buffer.trim().chars() {
                        seq.push_back(c);
                    }
                }
                1 => {
                    let parts: Vec<&str> = buffer.trim().split(" -> ").collect();
                    assert_eq!(parts.len(), 2);
                    assert_eq!(parts[0].len(), 2);
                    assert_eq!(parts[1].len(), 1);

                    let first = parts[0].chars().nth(0).unwrap();
                    let last = parts[0].chars().nth(1).unwrap();
                    let mid = parts[1].chars().nth(0).unwrap();

                    (*rule_tree.entry(first).or_insert(HashMap::new())).insert(last, mid);
                }
                _ => panic!("Invalid state"),
            }
            buffer.clear();
        }
        PolymerTemplate{seq, rule_tree}
    }

    fn render(&self) {
        for c in self.seq.iter() {
            print!("{}", c);
        }
        println!();
    }

    fn len(&self) -> usize {
        self.seq.len()
    }

    fn score(&self) -> usize {
        let mut el_counts: HashMap<char, usize> = HashMap::new();
        for c in self.seq.iter() {
            *el_counts.entry(*c).or_insert(0) += 1;
        }
        let most_common = el_counts.iter().reduce(|x, y| if x.1 >= y.1 { x } else { y }).unwrap().1;
        let least_common = el_counts.iter().reduce(|x, y| if x.1 <= y.1 { x } else { y }).unwrap().1;
        most_common - least_common
    }

    fn replacement_step(&mut self) {
        let mut cur = self.seq.cursor_front_mut();
        loop {
            let a = *cur.current().unwrap();
            match cur.peek_next() {
                Some(b) => {
                    if self.rule_tree.contains_key(&a) && self.rule_tree[&a].contains_key(&b) {
                        let c = self.rule_tree[&a][&b];
                        cur.insert_after(c);
                        cur.move_next();
                    }
                    cur.move_next();
                }
                None => {
                    break;
                }
            }
        }
    }
}

fn main() {
    let mut template = PolymerTemplate::parse();
    template.render();
    template.replacement_step();
    template.render();
    // For part 1, steps is 10. For part 2, 40
    let steps = 10;
    for i in 0..(steps - 1) {
        template.replacement_step();
    }
    println!("Length after {} steps: {}", steps, template.len());
    println!("Score after {} steps: {}", steps, template.score());
}
