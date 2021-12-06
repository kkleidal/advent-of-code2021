use std::io;

fn parse_starter_fish() -> Vec<u8> {
    let mut buffer = String::new();
    let mut starter_fish: Vec<u8> = Vec::new();
    loop {
        let n = io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read stdin");
        if n == 0 {
            // End of input
            break;
        }
        for val in buffer.trim().split(",") {
            let x: u8 = val.parse().expect("Invalid integer");
            starter_fish.push(x);
        }
        buffer.clear();
    }
    starter_fish
}

fn simulate_days(starter_counts: &[u64], days: usize) -> Vec<u64> {
    // println!("Starter counts: {:?}", counts);
    let mut counts: Vec<u64> = Vec::new();
    counts.extend_from_slice(starter_counts);
    for _ in 0..days {
        counts = vec![
            counts[1],
            counts[2],
            counts[3],
            counts[4],
            counts[5],
            counts[6],
            counts[7] + counts[0],
            counts[8],
            counts[0],
        ];
        // println!("After {} day(s), {} fish. Counts: {:?}", day + 1, counts.iter().sum::<u64>(), counts);
    }
    return counts;
}

fn main() {
    let starter_fish = parse_starter_fish();

    let mut counts: Vec<u64> = vec![0; 9];
    println!("Starters: {:?}", starter_fish);
    for fish in starter_fish {
        let offset: usize = fish.into();
        counts[offset] += 1;
    }
    println!("Starter counts: {:?}", counts);
    println!(
        "Part 1: {}",
        simulate_days(counts.as_slice(), 80).iter().sum::<u64>()
    );
    println!(
        "Part 2: {}",
        simulate_days(counts.as_slice(), 256).iter().sum::<u64>()
    );
}
