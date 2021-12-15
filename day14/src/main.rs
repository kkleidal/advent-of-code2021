use std::collections::HashMap;
use std::collections::LinkedList;
use std::io;

#[derive(Debug)]
struct PolymerTemplate {
    unigram_counts: HashMap<char, usize>,
    bigram_counts: HashMap<(char, char), usize>,
    rule_tree: HashMap<char, HashMap<char, char>>, // map first to second to middle
}

impl PolymerTemplate {
    fn parse() -> PolymerTemplate {
        let mut unigram_counts: HashMap<char, usize> = HashMap::new();
        let mut bigram_counts: HashMap<(char, char), usize> = HashMap::new();
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
                    let mut last: Option<char> = None;
                    for c in buffer.trim().chars() {
                        *(unigram_counts.entry(c).or_insert(0)) += 1;
                        match last {
                            None => {}
                            Some(last_c) => {
                                *(bigram_counts.entry((last_c, c)).or_insert(0)) += 1;
                            }
                        }
                        last = Some(c);
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
        PolymerTemplate {
            unigram_counts,
            bigram_counts,
            rule_tree,
        }
    }

    fn len(&self) -> usize {
        self.unigram_counts.values().sum()
    }

    fn score(&self) -> usize {
        let most_common = self
            .unigram_counts
            .iter()
            .reduce(|x, y| if x.1 >= y.1 { x } else { y })
            .unwrap()
            .1;
        let least_common = self
            .unigram_counts
            .iter()
            .reduce(|x, y| if x.1 <= y.1 { x } else { y })
            .unwrap()
            .1;
        most_common - least_common
    }

    fn replacement_step(&mut self) {
        for (key, count) in self
            .bigram_counts
            .iter()
            .map(|x| (*x.0, *x.1))
            .collect::<Vec<((char, char), usize)>>()
            .iter()
        {
            let a = key.0;
            let c = key.1;
            if self.rule_tree.contains_key(&a) {
                if self.rule_tree[&a].contains_key(&c) {
                    let b = self.rule_tree[&a][&c];
                    *(self.unigram_counts.entry(b).or_insert(0)) += count;
                    *(self.bigram_counts.entry((a, b)).or_insert(0)) += count;
                    *(self.bigram_counts.entry((b, c)).or_insert(0)) += count;
                    *(self.bigram_counts.entry((a, c)).or_insert(0)) -= count;
                }
            }
        }
    }
}

fn main() {
    let mut template = PolymerTemplate::parse();
    template.replacement_step();
    // For part 1, steps is 10. For part 2, 40
    let steps = 40;
    for i in 0..(steps - 1) {
        template.replacement_step();
    }
    println!("Length after {} steps: {}", steps, template.len());
    println!("Score after {} steps: {}", steps, template.score());
}
