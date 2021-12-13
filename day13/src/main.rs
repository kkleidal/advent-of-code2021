use std::collections::HashSet;
use std::io;

#[derive(Debug, Copy, Clone)]
enum FoldingInstruction {
    Y(i32),
    X(i32),
}

fn parse() -> (HashSet<(i32, i32)>, Vec<FoldingInstruction>) {
    let mut pairs: HashSet<(i32, i32)> = HashSet::new();
    let mut instructions: Vec<FoldingInstruction> = Vec::new();
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
                let parts: Vec<i32> = buffer
                    .trim()
                    .split(",")
                    .map(|x| x.parse::<i32>().expect("Invalid integer"))
                    .collect();
                assert_eq!(parts.len(), 2);
                // Convert from x,y to y, x
                pairs.insert((parts[1], parts[0]));
            }
            1 => {
                // fold along y=7
                let parts: Vec<&str> = buffer.trim()["fold along ".len()..].split("=").collect();
                assert_eq!(parts.len(), 2);
                let axis: i32 = parts[1].parse().expect("Invalid integer");
                instructions.push(match parts[0] {
                    "y" => FoldingInstruction::Y(axis),
                    "x" => FoldingInstruction::X(axis),
                    _ => panic!("Invalid axis"),
                });
            }
            _ => panic!("Invalid state"),
        }
        buffer.clear();
    }
    (pairs, instructions)
}

fn fold(pairs: &HashSet<(i32, i32)>, instruction: FoldingInstruction) -> HashSet<(i32, i32)> {
    let mut out: HashSet<(i32, i32)> = HashSet::new();
    for (yr, xr) in pairs.iter() {
        let y = *yr;
        let x = *xr;
        match instruction {
            FoldingInstruction::Y(axis) => {
                let new_y = if y > axis { 2 * axis - y } else { y };
                out.insert((new_y, x));
            }
            FoldingInstruction::X(axis) => {
                let new_x = if x > axis { 2 * axis - x } else { x };
                out.insert((y, new_x));
            }
        };
    }
    out
}

fn render(pairs: &HashSet<(i32, i32)>) {
    let max_y = pairs
        .iter()
        .map(|pair| pair.0)
        .max()
        .expect("Must be non-empty");
    let max_x = pairs
        .iter()
        .map(|pair| pair.1)
        .max()
        .expect("Must be non-empty");

    for y in -1..(max_y + 2) {
        for x in -1..(max_x + 2) {
            let key = (y, x);
            print!("{}", if pairs.contains(&key) { "#" } else { "." });
        }
        println!();
    }
    println!();
}

fn main() {
    let (pairs, instructions) = parse();
    println!("Part 1: {}", fold(&pairs, instructions[0]).len());
    let mut current_pairs = pairs;
    for inst in instructions {
        current_pairs = fold(&current_pairs, inst);
    }
    println!("Part 2:");
    render(&current_pairs);
}
