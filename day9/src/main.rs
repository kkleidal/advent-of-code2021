use std::collections::HashSet;
use std::collections::LinkedList;
use std::io;

fn parse_arr() -> Vec<Vec<i8>> {
    let mut buffer = String::new();
    let mut out: Vec<Vec<i8>> = Vec::new();
    loop {
        let n = io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read stdin");
        if n == 0 {
            // End of input
            break;
        }
        let row: Vec<i8> = buffer
            .trim()
            .chars()
            .map(|c| {
                let x: i8 = c.to_string().parse().expect("Invalid integer");
                x
            })
            .collect();
        out.push(row);
        buffer.clear();
    }
    out
}

fn adjacent(i: usize, j: usize, h: usize, w: usize) -> Vec<(usize, usize)> {
    let ii: isize = i.try_into().expect("Invalid conversion");
    let jj: isize = j.try_into().expect("Invalid conversion");
    let hh: isize = h.try_into().expect("Invalid conversion");
    let ww: isize = w.try_into().expect("Invalid conversion");
    let mut out: Vec<(usize, usize)> = Vec::new();
    for di in -1..2 {
        if ii + di < 0 || ii + di >= hh {
            continue;
        }
        for dj in -1..2 {
            if di == 0 && dj == 0 {
                continue;
            }
            if !(di == 0 || dj == 0) {
                continue;
            }
            if jj + dj < 0 || jj + dj >= ww {
                continue;
            }
            let iout: usize = (ii + di).try_into().expect("Invalid conversion");
            let jout: usize = (jj + dj).try_into().expect("Invalid conversion");
            out.push((iout, jout));
        }
    }
    out
}

fn find_low_points(heightmap: &[Vec<i8>]) -> Vec<(usize, usize)> {
    let mut out: Vec<(usize, usize)> = Vec::new();
    let h = heightmap.len();
    let w = heightmap[0].len();
    for i in 0..h {
        for j in 0..w {
            let mut lesser = true;
            for (ii, jj) in adjacent(i, j, h, w) {
                if heightmap[ii][jj] <= heightmap[i][j] {
                    lesser = false;
                }
            }
            if lesser {
                out.push((i, j));
            }
        }
    }
    out
}

fn part1(heightmap: &[Vec<i8>]) -> u64 {
    let mut out: u64 = 0;
    for (i, j) in find_low_points(heightmap) {
        let val: u64 = heightmap[i][j].try_into().expect("Invalid conversion");
        out += val + 1;
    }
    out
}

fn part2(heightmap: &[Vec<i8>]) -> usize {
    let h = heightmap.len();
    let w = heightmap[0].len();
    let mut basin_sizes: Vec<usize> = Vec::new();
    for (i, j) in find_low_points(heightmap) {
        let mut stack: LinkedList<(usize, usize)> = LinkedList::new();
        let mut basin: HashSet<(usize, usize)> = HashSet::new();
        basin.insert((i, j));
        stack.push_back((i, j));
        while stack.len() > 0 {
            let (i, j) = stack.pop_back().expect("Empty stack");
            for (ii, jj) in adjacent(i, j, h, w) {
                let val = heightmap[ii][jj];
                let source = heightmap[i][j];
                let pt = (ii, jj);
                if val >= source && val != 9 && !basin.contains(&pt) {
                    basin.insert(pt);
                    stack.push_back(pt);
                }
            }
        }
        basin_sizes.push(basin.len());
    }
    basin_sizes.sort();
    basin_sizes[basin_sizes.len() - 3..]
        .into_iter()
        .map(|x| *x)
        .reduce(|x, y| x * y)
        .expect("No values")
}

fn main() {
    let heightmap = parse_arr();
    println!("Part 1: {}", part1(heightmap.as_slice()));
    println!("Part 2: {}", part2(heightmap.as_slice()));
}
