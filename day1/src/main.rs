use std::io;

fn parse_input() -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();
    let mut buffer = String::new();
    loop {
        let n = io::stdin().read_line(&mut buffer).expect("Failed to read stdin");
        if n == 0 {
            // End of input
            break;
        }
        let x: i32 = buffer.trim().parse().expect("Invalid integer");
        out.push(x);
        buffer.clear();
    }
    out
}

fn number_increases(values: &[i32]) -> usize {
    let mut increases: usize = 0;
    let mut prev: i32 = 0;
    let mut first = true;
    for &x in values {
        if !first && x > prev {
            increases += 1;
        }
        first = false;
        prev = x;
    }
    increases
}

fn convolve3(values: &[i32]) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();
    for i in 0..values.len() - 2 {
        out.push(values[i] + values[i+1] + values[i+2]);
    }
    out
}

fn main() {
    let input = parse_input();
    let increases = number_increases(&input[..]);
    println!("Part 1: {}", increases);
    let windows = convolve3(&input[..]);
    let window_increases = number_increases(&windows[..]);
    println!("Part 2: {}", window_increases);
}
