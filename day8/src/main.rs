use std::io;

#[derive(Debug)]
struct Signal {
    digits: String,
}

impl Signal {
    fn len(&self) -> usize {
        self.digits.len()
    }
}

fn parse_signals_part(val: &str) -> Vec<Signal> {
    let mut signals: Vec<Signal> = Vec::new();
    for seg in val.split(" ") {
        let mut seg_str: String = String::from(seg);
        let mut chars: Vec<char> = seg_str.chars().collect();
        chars.sort_by(|a, b| b.cmp(a));
        seg_str = String::from_iter(chars);
        signals.push(Signal { digits: seg_str });
    }
    signals
}

fn parse_signals() -> Vec<(Vec<Signal>, Vec<Signal>)> {
    let mut buffer = String::new();
    let mut out: Vec<(Vec<Signal>, Vec<Signal>)> = Vec::new();
    loop {
        let n = io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read stdin");
        if n == 0 {
            // End of input
            break;
        }
        let buffer_parts: Vec<&str> = buffer.trim().split(" | ").collect();
        assert!(buffer_parts.len() == 2);
        out.push((
            parse_signals_part(buffer_parts[0]),
            parse_signals_part(buffer_parts[1]),
        ));
        buffer.clear();
    }
    out
}

fn part1(signals: &[(Vec<Signal>, Vec<Signal>)]) -> usize {
    let mut count: usize = 0;
    for (_, out_sig) in signals {
        for sig in out_sig {
            let n = sig.len();
            if n == 2 || n == 4 || n == 3 || n == 7 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let signals = parse_signals();
    println!("Part 1: {}", part1(signals.as_slice()));
}
