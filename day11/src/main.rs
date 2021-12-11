use std::collections::LinkedList;
use std::io;

fn parse() -> Vec<Vec<u8>> {
    let mut buffer = String::new();
    let mut out: Vec<Vec<u8>> = Vec::new();
    loop {
        let n = io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read stdin");
        if n == 0 {
            // End of input
            break;
        }
        let row: Vec<u8> = buffer
            .trim()
            .chars()
            .map(|c| c.to_string().parse::<u8>().expect("Invalid integer"))
            .collect();
        out.push(row);
        buffer.clear();
    }
    out
}

fn simulate(grid: &mut [Vec<u8>]) -> usize {
    let h: isize = grid.len().try_into().expect("Invalid conversion");
    let w: isize = grid[0].len().try_into().expect("Invalid conversion");
    let mut ready: LinkedList<(isize, isize)> = LinkedList::new();
    let t = |x: isize| {
        let y: usize = x.try_into().expect("Invalid conversion");
        y
    };
    for i in 0..h {
        for j in 0..w {
            grid[t(i)][t(j)] += 1;
            if grid[t(i)][t(j)] == 10 {
                ready.push_back((i, j));
            }
        }
    }
    let mut flashes = 0;
    while ready.len() > 0 {
        let (i, j) = ready.pop_front().expect("Must not be empty");
        grid[t(i)][t(j)] = 0;
        flashes += 1;
        for di in -1..2 {
            let ii = di + i;
            if ii < 0 || ii >= h {
                continue;
            }
            for dj in -1..2 {
                let jj = dj + j;
                if jj < 0 || jj >= w || (di == 0 && dj == 0) {
                    continue;
                }
                if grid[t(ii)][t(jj)] != 0 && grid[t(ii)][t(jj)] != 10 {
                    // Otherwise, already flashed this round
                    grid[t(ii)][t(jj)] += 1;
                    if grid[t(ii)][t(jj)] == 10 {
                        ready.push_back((ii, jj));
                    }
                }
            }
        }
    }
    flashes
}

fn main() {
    let grid = parse();
    let mut part1_grid = grid.clone();
    let mut n_flashes = 0;
    for _ in 0..100 {
        n_flashes += simulate(part1_grid.as_mut_slice());
    }
    println!("Part 1: {}", n_flashes);

    let total = grid.len() * grid[0].len();
    // Hack: assumes synchronization doesn't occur in first 100 steps:
    let mut t = 101;
    loop {
        let n_flashes = simulate(part1_grid.as_mut_slice());
        if n_flashes == total {
            break;
        }
        t += 1;
    }
    println!("Part 2: {}", t);
}
