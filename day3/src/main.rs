use std::io;

#[derive(Debug)]
struct Reading {
    digits: Vec<u8>,
}

impl Reading {
    fn to_decimal(&self) -> u64 {
        let mut value: u64 = 0;
        for i in 0..self.digits.len() {
            let shift: u64 = (self.digits.len() - i - 1).try_into().unwrap();
            let digit: u64 = self.digits[i].into();
            value |= digit << shift;
        }
        value
    }
}

fn parse_input() -> Vec<Reading> {
    let mut out: Vec<Reading> = Vec::new();
    let mut buffer = String::new();
    loop {
        let n = io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read stdin");
        if n == 0 {
            // End of input
            break;
        }
        let mut row: Vec<u8> = Vec::new();
        for c in buffer.trim().chars() {
            let x: u8 = match c {
                '0' => 0,
                '1' => 1,
                _ => panic!("Invalid character {}", c),
            };
            row.push(x);
        }
        out.push(Reading { digits: row });
        buffer.clear();
    }
    out
}

fn ones_counts(readings: &[Reading]) -> Vec<u64> {
    let mut ones: Vec<u64> = vec![0; readings[0].digits.len()];
    for reading in readings {
        for d in 0..reading.digits.len() {
            if reading.digits[d] == 1 {
                ones[d] += 1;
            }
        }
    }
    ones
}

fn gamma_rate(readings: &[Reading]) -> u64 {
    let ones = ones_counts(readings);
    let n: u64 = readings.len().try_into().unwrap();
    let mut new_reading_digits: Vec<u8> = Vec::new();
    for i in 0..ones.len() {
        new_reading_digits.push(if ones[i] > (n - ones[i]) { 1 } else { 0 })
    }
    let new_reading = Reading {
        digits: new_reading_digits,
    };
    new_reading.to_decimal()
}

fn epsilon_rate(readings: &[Reading]) -> u64 {
    let ones = ones_counts(readings);
    let n: u64 = readings.len().try_into().unwrap();
    let mut new_reading_digits: Vec<u8> = Vec::new();
    for i in 0..ones.len() {
        new_reading_digits.push(if ones[i] <= (n - ones[i]) { 1 } else { 0 })
    }
    let new_reading = Reading {
        digits: new_reading_digits,
    };
    new_reading.to_decimal()
}

fn oxy_co2_rate(readings: &[Reading], oxy: bool) -> u64 {
    let mut mask: Vec<bool> = vec![true; readings.len()];
    for i in 0..readings[0].digits.len() {
        let mut ones: usize = 0;
        let mut count: usize = 0;
        for j in 0..readings.len() {
            if mask[j] {
                if readings[j].digits[i] == 1 {
                    ones += 1;
                }
                count += 1;
            }
        }
        let keep: u8 = {
            let val = if ones >= (count - ones) { 1 } else { 0 };
            if oxy {
                val
            } else {
                1 - val
            }
        };
        count = 0;
        let mut found_index: usize = 0;
        for j in 0..readings.len() {
            if mask[j] {
                if readings[j].digits[i] == keep {
                    count += 1;
                    found_index = j;
                } else {
                    mask[j] = false;
                }
            }
        }
        if count == 1 {
            return readings[found_index].to_decimal();
        }
    }
    panic!("No definitive solution");
}

fn main() {
    let inp = parse_input();
    println!("Part 1: {:?}", gamma_rate(&inp) * epsilon_rate(&inp));
    println!(
        "Part 2: {:?}",
        oxy_co2_rate(&inp, true) * oxy_co2_rate(&inp, false)
    );
}
