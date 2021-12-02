use std::io;

// #[derive(Debug)]
enum Direction {
    Up,
    Down,
    Forward,
}

fn parse_input() -> Vec<(Direction, i64)> {
    let mut out: Vec<(Direction, i64)> = Vec::new();
    let mut buffer = String::new();
    loop {
        let n = io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read stdin");
        if n == 0 {
            // End of input
            break;
        }
        let parts: Vec<&str> = buffer.trim().split(" ").collect();
        assert_eq!(parts.len(), 2);
        let direction = match parts[0] {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => panic!("Invalid direction: {}", parts[0]),
        };
        let distance: i64 = parts[1].parse().expect("Invalid integer");
        out.push((direction, distance));
        buffer.clear();
    }
    out
}

fn get_final_position(directions: &[(Direction, i64)]) -> (i64, i64) {
    let mut horiz: i64 = 0;
    let mut depth: i64 = 0;
    for (direction, distance) in directions {
        match direction {
            Direction::Up => depth -= distance,
            Direction::Down => depth += distance,
            Direction::Forward => horiz += distance,
        }
    }
    return (horiz, depth);
}

fn get_final_position_with_aim(directions: &[(Direction, i64)]) -> (i64, i64) {
    let mut horiz: i64 = 0;
    let mut depth: i64 = 0;
    let mut aim: i64 = 0;
    for (direction, distance) in directions {
        match direction {
            Direction::Up => aim -= distance,
            Direction::Down => aim += distance,
            Direction::Forward => {
                horiz += distance;
                depth += aim * distance;
            }
        }
    }
    return (horiz, depth);
}

fn main() {
    let inp = parse_input();
    let (horiz, depth) = get_final_position(&inp[..]);
    println!("Part 1: {}", horiz * depth);
    let (horiz, depth) = get_final_position_with_aim(&inp[..]);
    println!("Part 2: {}", horiz * depth);
}
