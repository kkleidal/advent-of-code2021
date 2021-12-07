use std::io;

fn parse_crabs() -> Vec<i64> {
    let mut buffer = String::new();
    let mut crabs: Vec<i64> = Vec::new();
    loop {
        let n = io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read stdin");
        if n == 0 {
            // End of input
            break;
        }
        for val in buffer.trim().split(",") {
            let x: i64 = val.parse().expect("Invalid integer");
            crabs.push(x);
        }
        buffer.clear();
    }
    crabs
}

fn part1(crabs: &[i64]) -> i64 {
    let median = crabs[crabs.len() / 2];
    let cost = crabs.iter().map(|x| (x - median).abs()).sum::<i64>();
    cost
}

fn part2(crabs: &[i64]) -> i64 {
    let mut prev_cost: i64 = 0;
    for i in 0..crabs.len() {
        let trial = crabs[i];
        let cost = crabs
            .iter()
            .map(|x| {
                let diff = (x - trial).abs();
                diff * (diff + 1) / 2
            })
            .sum::<i64>();
        if i != 0 && cost > prev_cost {
            break;
        }
        prev_cost = cost;
    }
    prev_cost
}

fn main() {
    let mut crabs = parse_crabs();
    crabs.sort();
    println!("Crabs: {:?}", crabs);
    println!("Part 1: {}", part1(crabs.as_slice()));
    println!("Part 2: {}", part2(crabs.as_slice()));
}
