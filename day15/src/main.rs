use rudac::heap::FibonacciHeap;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::io;

#[derive(Debug)]
struct Cave {
    plot: Vec<Vec<usize>>,
    height: usize,
    width: usize,
}

impl Cave {
    fn parse() -> Cave {
        let mut plot: Vec<Vec<usize>> = Vec::new();
        let mut buffer = String::new();
        loop {
            let n = io::stdin()
                .read_line(&mut buffer)
                .expect("Failed to read stdin");
            if n == 0 {
                // End of input
                break;
            }
            let row = buffer
                .trim()
                .chars()
                .map(|c| c.to_string().parse::<usize>().expect("Invalid character"))
                .collect();
            plot.push(row);
            buffer.clear();
        }
        let height = plot.len();
        let width = plot[0].len();
        Cave {
            plot,
            height,
            width,
        }
    }

    fn neighbors(&self, node: (usize, usize), diags: bool) -> Vec<(usize, usize)> {
        let mut neighbors: Vec<(usize, usize)> = Vec::new();
        let mut ys: Vec<usize> = Vec::new();
        let mut xs: Vec<usize> = Vec::new();
        if node.0 > 0 {
            ys.push(node.0 - 1);
        }
        ys.push(node.0);
        if node.0 + 1 < self.height {
            ys.push(node.0 + 1);
        }
        if node.1 > 0 {
            xs.push(node.1 - 1);
        }
        xs.push(node.1);
        if node.1 + 1 < self.width {
            xs.push(node.1 + 1);
        }
        for y in ys.iter() {
            for x in xs.iter() {
                if !diags && (*y != node.0 && *x != node.1) {
                    continue;
                }
                if !(*y == node.0 && *x == node.1) {
                    neighbors.push((*y, *x));
                }
            }
        }
        neighbors
    }

    fn times(&self, multiplier: usize) -> Cave {
        let mut new_plot: Vec<Vec<usize>> = Vec::new();
        for i in 0..(self.height * multiplier) {
            let mut new_row: Vec<usize> = Vec::new();
            let orig_i = i % self.height;
            let tile_i = i / self.height;
            for j in 0..(self.width * multiplier) {
                let orig_j = j % self.width;
                let tile_j = j / self.width;
                let orig_val = self.plot[orig_i][orig_j];
                let new_val = (orig_val + tile_i + tile_j - 1) % 9 + 1;
                new_row.push(new_val);
            }
            new_plot.push(new_row);
        }
        Cave {
            plot: new_plot,
            height: self.height * multiplier,
            width: self.width * multiplier,
        }
    }

    fn heuristic(&self, node: (usize, usize)) -> usize {
        // Manhattan distance
        self.height - 1 - node.0 + self.width - 1 - node.1
    }

    fn lowest_risk_path(&self) -> usize {
        // Djikstra (with a star heuristic)
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let start: (usize, usize) = (0, 0);
        let end = (self.height - 1, self.width - 1);
        let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
        let mut q: FibonacciHeap<Node> = FibonacciHeap::init_max();
        q.push(Node {
            position: start,
            cost: self.heuristic(start),
        });
        distances.insert(start, 0);
        loop {
            let popped = q.pop().expect("Empty queue");
            let current = popped.position;
            if current == end {
                break;
            }
            if visited.contains(&current) {
                continue;
            }

            let current_dist = distances[&current];
            for neighbor in self.neighbors(current, false) {
                if !visited.contains(&neighbor) {
                    let possible = current_dist + self.plot[neighbor.0][neighbor.1];
                    if !distances.contains_key(&neighbor) || distances[&neighbor] > possible {
                        distances.insert(neighbor, possible);
                        q.push(Node {
                            position: neighbor,
                            cost: possible + self.heuristic(neighbor),
                        });
                    }
                }
            }
            visited.insert(current);
        }
        distances[&end]
    }
}

#[derive(Debug)]
struct Node {
    cost: usize,
    position: (usize, usize),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.cost, self.position)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.position == other.position
    }
}

impl Eq for Node {}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let cave = Cave::parse();
    println!("Lowest risk in 1x: {}", cave.lowest_risk_path());
    let big_cave = cave.times(5);
    println!("Lowest risk in 5x: {}", big_cave.lowest_risk_path());
}
