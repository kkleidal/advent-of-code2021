use std::cmp::max;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::io;

#[derive(Debug)]
struct Node {
    name: String,
    big: bool,
}

#[derive(Debug)]
struct Graph {
    start: usize,
    end: usize,
    nodes: Vec<Node>,
    edges: Vec<(usize, usize)>,
    neighbors: Vec<Vec<usize>>,
}

impl Graph {
    fn is_big(&self, index: usize) -> bool {
        self.nodes[index].big
    }

    fn parse() -> Graph {
        let mut nodes: Vec<Node> = Vec::new();
        let mut name_to_node: HashMap<String, usize> = HashMap::new();
        let mut edges: Vec<(usize, usize)> = Vec::new();

        let mut buffer = String::new();
        loop {
            let n = io::stdin()
                .read_line(&mut buffer)
                .expect("Failed to read stdin");
            if n == 0 {
                // End of input
                break;
            }
            let row: Vec<usize> = buffer
                .trim()
                .split("-")
                .map(|part| {
                    if !name_to_node.contains_key(part) {
                        let big = part.to_uppercase() == part;
                        nodes.push(Node {
                            name: part.to_string(),
                            big,
                        });
                        name_to_node.insert(part.to_string(), nodes.len() - 1);
                    }
                    name_to_node[part]
                })
                .collect();
            assert_eq!(row.len(), 2);
            edges.push((row[0], row[1]));
            buffer.clear();
        }

        let mut neighbors: Vec<Vec<usize>> = nodes.iter().map(|_| Vec::new()).collect();
        for (i, j) in edges.iter() {
            neighbors[*i].push(*j);
            neighbors[*j].push(*i);
        }

        Graph {
            start: name_to_node["start"],
            end: name_to_node["end"],
            nodes,
            edges,
            neighbors,
        }
    }
}

fn part1(graph: &Graph) -> usize {
    let mut paths: LinkedList<Vec<usize>> = LinkedList::new();
    paths.push_back(vec![graph.start]);
    let mut terminal: usize = 0;
    while paths.len() > 0 {
        let path = paths.pop_back().expect("Must not be empty");
        let last = path[path.len() - 1];
        if last == graph.end {
            terminal += 1;
            continue;
        }
        for neighbor in graph.neighbors[last].iter() {
            if graph.is_big(*neighbor) || !path.contains(neighbor) {
                let mut new_path = path.clone();
                new_path.push(*neighbor);
                paths.push_back(new_path);
            }
        }
    }
    terminal
}

fn max_unique_small_cave_visits(graph: &Graph, path: &[usize]) -> usize {
    let mut counts: HashMap<usize, usize> = HashMap::new();
    let mut max_found = 0;
    for node in path.iter() {
        if !graph.is_big(*node) {
            if !counts.contains_key(node) {
                counts.insert(*node, 0);
            }
            counts.insert(*node, counts[node] + 1);
            max_found = max(counts[node], max_found);
        }
    }
    max_found
}

fn part2(graph: &Graph) -> usize {
    let mut paths: LinkedList<Vec<usize>> = LinkedList::new();
    paths.push_back(vec![graph.start]);
    let mut terminal: usize = 0;
    while paths.len() > 0 {
        let path = paths.pop_back().expect("Must not be empty");
        let last = path[path.len() - 1];
        if last == graph.end {
            terminal += 1;
            continue;
        }
        for neighbor in graph.neighbors[last].iter() {
            if graph.is_big(*neighbor)
                || !path.contains(neighbor)
                || (*neighbor != graph.start
                    && max_unique_small_cave_visits(graph, &path.as_slice()) < 2)
            {
                let mut new_path = path.clone();
                new_path.push(*neighbor);
                paths.push_back(new_path);
            }
        }
    }
    terminal
}

fn main() {
    let graph = Graph::parse();
    println!("Graph: {:?}", graph);
    println!("Part 1: {}", part1(&graph));
    println!("Part 2: {}", part2(&graph));
}
