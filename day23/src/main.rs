use rudac::heap::FibonacciHeap;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, LinkedList};
use std::hash::{Hash, Hasher};
use std::io;

#[derive(Debug, Clone, Copy)]
enum NodeKind {
    Hallway,
    HallwayDoor,
    Room,
}

#[derive(Debug)]
struct Node {
    y: usize,
    x: usize,
    kind: NodeKind,
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    adjacency: HashMap<usize, HashSet<usize>>,
}

impl Graph {
    fn amph_dist_to_home(&self, node_id: usize, amph_kind: usize) -> usize {
        let mut room_xs: HashSet<usize> = HashSet::new();
        for node in self.nodes.iter() {
            match node.kind {
                NodeKind::Room => {
                    room_xs.insert(node.x);
                }
                _ => (),
            }
        }
        let mut room_xs: Vec<usize> = room_xs.into_iter().collect();
        room_xs.sort();
        let dest_x = room_xs[amph_kind];

        // BFS
        let mut queue: LinkedList<(usize, usize)> = LinkedList::new();
        let mut visited: HashSet<usize> = HashSet::new();
        queue.push_back((node_id, 0));
        visited.insert(node_id);
        while queue.len() > 0 {
            let (node, dist) = queue.pop_front().unwrap();
            let node_x = self.nodes[node].x;
            if node_x == dest_x {
                match self.nodes[node].kind {
                    NodeKind::Room => {
                        return dist;
                    }
                    _ => (),
                }
            }
            for adj in self.adjacency[&node].iter() {
                if !visited.contains(&adj) {
                    queue.push_back((*adj, dist + 1));
                    visited.insert(*adj);
                }
            }
        }
        panic!("No path found");
    }

    fn amph_is_home(&self, node_id: usize, amph_kind: usize) -> bool {
        let node = &self.nodes[node_id];
        match node.kind {
            NodeKind::Room => (),
            _ => {
                return false;
            }
        }

        let mut room_xs: HashSet<usize> = HashSet::new();
        for node in self.nodes.iter() {
            match node.kind {
                NodeKind::Room => {
                    room_xs.insert(node.x);
                }
                _ => (),
            }
        }
        let mut room_xs: Vec<usize> = room_xs.into_iter().collect();
        room_xs.sort();
        let dest_x = room_xs[amph_kind];
        let node_x = node.x;
        dest_x == node_x
    }
}

#[derive(Debug, Clone, Copy)]
enum AmphipodState {
    Unmoved,
    MovedOut,
    MovedIn,
}

impl PartialEq for AmphipodState {
    fn eq(&self, other: &Self) -> bool {
        match self {
            AmphipodState::Unmoved => match other {
                AmphipodState::Unmoved => true,
                _ => false,
            },
            AmphipodState::MovedOut => match other {
                AmphipodState::MovedOut => true,
                _ => false,
            },
            AmphipodState::MovedIn => match other {
                AmphipodState::MovedIn => true,
                _ => false,
            },
        }
    }
}

impl Eq for AmphipodState {}

impl Hash for AmphipodState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            AmphipodState::Unmoved => 0_u8.hash(state),
            AmphipodState::MovedOut => 1_u8.hash(state),
            AmphipodState::MovedIn => 2_u8.hash(state),
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    node_to_amphipod: HashMap<usize, (usize, AmphipodState)>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        if self.node_to_amphipod.len() != other.node_to_amphipod.len() {
            return false;
        }
        for (key, val1) in self.node_to_amphipod.iter() {
            if !other.node_to_amphipod.contains_key(key) {
                return false;
            }
            if other.node_to_amphipod[key] != *val1 {
                return false;
            }
        }
        true
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node_to_amphipod.len().hash(state);
        for (key, val) in self.node_to_amphipod.iter() {
            key.hash(state);
            val.hash(state);
        }
    }
}

impl State {
    fn finished(&self, graph: &Graph) -> bool {
        let mut x_coords: Vec<usize> = vec![0, 0, 0, 0];
        for (node_id, node) in graph.nodes.iter().enumerate() {
            match node.kind {
                NodeKind::Hallway => (),
                NodeKind::HallwayDoor => (),
                NodeKind::Room => {
                    for back_node_id in find_room_stack(graph, node_id) {
                        if self.node_to_amphipod.contains_key(&back_node_id)
                            && self.node_to_amphipod.contains_key(&node_id)
                        {
                            let amph_kind = self.node_to_amphipod[&back_node_id].0;
                            if amph_kind == self.node_to_amphipod[&node_id].0 {
                                x_coords[amph_kind] = node.x;
                                // Good
                            } else {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                }
            }
        }
        if x_coords[3] > x_coords[2] && x_coords[2] > x_coords[1] && x_coords[1] > x_coords[0] {
            return true;
        } else {
            return false;
        }
    }
}

fn parse() -> (Graph, State) {
    let mut buffer = String::new();
    let mut grid: Vec<Vec<(Option<NodeKind>, Option<usize>)>> = Vec::new();
    loop {
        let n = io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read stdin");
        if n == 0 {
            // End of input
            break;
        }
        let row: Vec<(Option<NodeKind>, Option<usize>)> = buffer
            .chars()
            .filter(|c| *c != '\n' && *c != '\r')
            .map(|c| match c {
                '#' => (None, None),
                ' ' => (None, None),
                '.' => (Some(NodeKind::Hallway), None),
                'A' => (Some(NodeKind::Room), Some(0)),
                'B' => (Some(NodeKind::Room), Some(1)),
                'C' => (Some(NodeKind::Room), Some(2)),
                'D' => (Some(NodeKind::Room), Some(3)),
                _ => panic!("Unknown character {}", c),
            })
            .collect();
        grid.push(row);
        buffer.clear();
    }

    let height = grid.len();
    let mut nodes: Vec<Node> = Vec::new();
    let mut loc_to_node: HashMap<(isize, isize), usize> = HashMap::new();
    for y in 0..height {
        let width = grid[y].len();
        for x in 0..width {
            let (kind_opt, _) = &grid[y][x];
            match kind_opt {
                None => (),
                Some(kind) => {
                    let my_kind = match kind {
                        NodeKind::Hallway => {
                            let below = if grid[y + 1].len() > x {
                                grid[y + 1][x].0.clone()
                            } else {
                                None
                            };
                            let my_kind = match below {
                                Some(_) => NodeKind::HallwayDoor,
                                None => NodeKind::Hallway,
                            };
                            my_kind
                        }
                        NodeKind::Room => NodeKind::Room,
                        _ => panic!("Unexpected value"),
                    };
                    let node_id = nodes.len();
                    nodes.push(Node {
                        y,
                        x,
                        kind: my_kind,
                    });
                    loc_to_node.insert((y.try_into().unwrap(), x.try_into().unwrap()), node_id);
                }
            }
        }
    }

    let mut adjacency: HashMap<usize, HashSet<usize>> = HashMap::new();
    for ((y, x), node_id) in loc_to_node.iter() {
        adjacency.insert(*node_id, HashSet::new());
        for (dy, dx) in vec![(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let y2 = dy + y;
            let x2 = dx + x;
            let key = (y2, x2);
            if loc_to_node.contains_key(&key) {
                let neighbor = loc_to_node[&key];
                adjacency
                    .entry(*node_id)
                    .or_insert(HashSet::new())
                    .insert(neighbor);
            }
        }
    }

    let mut node_to_amphipod: HashMap<usize, (usize, AmphipodState)> = HashMap::new();
    for ((y, x), node_id) in loc_to_node.iter() {
        let yu: usize = (*y).try_into().unwrap();
        let xu: usize = (*x).try_into().unwrap();
        match grid[yu][xu].1 {
            Some(amphipod_type) => {
                node_to_amphipod.insert(*node_id, (amphipod_type, AmphipodState::Unmoved));
            }
            None => (),
        }
    }

    let graph = Graph { nodes, adjacency };
    let state = State { node_to_amphipod };

    (graph, state)
}

fn find_accessible_nodes(
    graph: &Graph,
    state: &State,
    node_id: usize,
    amphipod_type: usize,
) -> Vec<(usize, usize)> {
    let mut out: Vec<(usize, usize)> = Vec::new();
    let mut stack: LinkedList<(usize, usize)> = LinkedList::new();
    let mut will_visit: HashSet<usize> = HashSet::new();
    stack.push_back((node_id, 0));
    will_visit.insert(node_id);
    while stack.len() > 0 {
        let (current_node, cost) = stack.pop_back().unwrap();
        if current_node != node_id {
            out.push((current_node, cost));
        }
        for neighbor in graph.adjacency[&current_node].iter() {
            if will_visit.contains(&neighbor) {
                continue;
            }
            if state.node_to_amphipod.contains_key(&neighbor) {
                continue;
            }
            will_visit.insert(*neighbor);
            stack.push_back((*neighbor, cost + 10_usize.pow(amphipod_type as u32)));
        }
    }
    out
}

fn find_room_stack(graph: &Graph, node_id: usize) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();
    let my_node = &graph.nodes[node_id];
    let my_x = my_node.x;
    for (i, node) in graph.nodes.iter().enumerate() {
        if node.x == my_x {
            match node.kind {
                NodeKind::Room => {
                    out.push(i);
                }
                _ => (),
            }
        }
    }
    out
}

fn find_room_stack_below(graph: &Graph, node_id: usize) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();
    let my_node = &graph.nodes[node_id];
    let my_x = my_node.x;
    let my_y = my_node.y;
    for (i, node) in graph.nodes.iter().enumerate() {
        if node.x == my_x && node.y > my_y {
            match node.kind {
                NodeKind::Room => {
                    out.push(i);
                }
                _ => (),
            }
        }
    }
    out
}

// fn find_adjacent_back(graph: &Graph, node_id: usize) -> Option<usize> {
//     for adj in graph.adjacency[&node_id].iter() {
//         let node = &graph.nodes[*adj];
//         if let NodeKind::RoomBack = node.kind {
//             return Some(*adj);
//         }
//     }
//     None
// }

fn possible_next_states(graph: &Graph, state: &State) -> Vec<(State, usize)> {
    let mut costs: Vec<(State, usize)> = Vec::new();
    for (node_id, (amphipod_type, amphi_state)) in state.node_to_amphipod.iter() {
        match amphi_state {
            AmphipodState::Unmoved => {
                for (dest_id, cost) in find_accessible_nodes(graph, state, *node_id, *amphipod_type)
                {
                    let dest = &graph.nodes[dest_id];
                    match dest.kind {
                        NodeKind::Hallway => {
                            let mut new_node_to_amphipod = state.node_to_amphipod.clone();
                            new_node_to_amphipod.remove(node_id);
                            new_node_to_amphipod
                                .insert(dest_id, (*amphipod_type, AmphipodState::MovedOut));
                            costs.push((
                                State {
                                    node_to_amphipod: new_node_to_amphipod,
                                },
                                cost,
                            ));
                        }
                        NodeKind::HallwayDoor => (),
                        NodeKind::Room => (),
                    }
                }
            }
            AmphipodState::MovedOut => {
                for (dest_id, cost) in find_accessible_nodes(graph, state, *node_id, *amphipod_type)
                {
                    let dest = &graph.nodes[dest_id];
                    match dest.kind {
                        NodeKind::Hallway => (),
                        NodeKind::HallwayDoor => (),
                        NodeKind::Room => {
                            if graph.amph_is_home(dest_id, *amphipod_type) {
                                let mut valid: bool = true;
                                for back_id in find_room_stack_below(graph, dest_id) {
                                    if !state.node_to_amphipod.contains_key(&back_id) {
                                        // Invalid, can't have blank spaces below
                                        valid = false;
                                        break;
                                    }
                                    let neighbor_amphi_type = state.node_to_amphipod[&back_id].0;
                                    if neighbor_amphi_type != *amphipod_type {
                                        // Invalid, can't mix amphipod types
                                        valid = false;
                                        break;
                                    }
                                }
                                if valid {
                                    // Valid placement
                                    let mut new_node_to_amphipod = state.node_to_amphipod.clone();
                                    new_node_to_amphipod.remove(node_id);
                                    new_node_to_amphipod
                                        .insert(dest_id, (*amphipod_type, AmphipodState::MovedIn));
                                    costs.push((
                                        State {
                                            node_to_amphipod: new_node_to_amphipod,
                                        },
                                        cost,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            AmphipodState::MovedIn => (),
        }
    }
    costs
}

#[derive(Debug, Clone)]
struct StateEntry {
    cost: usize,
    state: State,
    // pp: Vec<StateEntry>,
}

impl PartialEq for StateEntry {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.state == other.state
    }
}

impl Eq for StateEntry {}

impl Ord for StateEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for StateEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic(graph: &Graph, initial_state: &State) -> usize {
    // Distance between like amphipods
    // let mut amphipod_to_node: HashMap<usize, HashSet<usize>> = HashMap::new();
    // for (node_id, (amphi_type, _)) in initial_state.node_to_amphipod.iter() {
    //     amphipod_to_node.entry(*amphi_type).or_insert(HashSet::new()).insert(*node_id);
    // }
    // let mut cost: usize = 0;
    // for (amphi_type, nodes) in amphipod_to_node.iter() {
    //     assert_eq!(nodes.len(), 2);
    //     let v: Vec<usize> = nodes.into_iter().cloned().collect();
    //     let p1 = v[0];
    //     let p2 = v[1];
    //     cost += 10_usize.pow(*amphi_type as u32) * (graph.distance_between(p1, p2) - 1);
    // }
    // cost

    // Distance between amphipod and destination
    let mut cost: usize = 0;
    for (node_id, (amphi_type, _)) in initial_state.node_to_amphipod.iter() {
        cost += 10_usize.pow(*amphi_type as u32) * graph.amph_dist_to_home(*node_id, *amphi_type);
    }
    cost
}

fn shortest_path(graph: &Graph, initial_state: &State) -> Option<usize> {
    // Djikstra
    let mut visited: HashSet<State> = HashSet::new();
    let mut distances: HashMap<State, usize> = HashMap::new();
    let mut q: FibonacciHeap<StateEntry> = FibonacciHeap::init_max();
    q.push(StateEntry {
        state: initial_state.clone(),
        cost: heuristic(graph, initial_state),
        // pp: Vec::new(),
    });
    distances.insert(initial_state.clone(), 0);
    loop {
        let popped = match q.pop() {
            Some(x) => x,
            None => break,
        };
        let current = &popped.state;
        let current_dist = distances[current];
        // println!("Queue size: {}, min cost: {}", q.size(), current_dist + heuristic(graph, current));
        if current.finished(graph) {
            return Some(distances[current]);
        }
        if visited.contains(current) {
            continue;
        }
        for (neighbor, cost) in possible_next_states(graph, current) {
            if !visited.contains(&neighbor) {
                let possible = current_dist + cost;
                if !distances.contains_key(&neighbor) || distances[&neighbor] > possible {
                    let cost = possible + heuristic(graph, &neighbor);
                    distances.insert(neighbor.clone(), possible);
                    // let mut pp = popped.pp.clone();
                    // pp.push(popped.clone());
                    q.push(StateEntry {
                        state: neighbor,
                        cost,
                        // pp: pp,
                    });
                }
            }
        }
        visited.insert(current.clone());
    }
    None
}

fn main() {
    let (graph, state) = parse();
    println!("Costs: {}", shortest_path(&graph, &state).unwrap());
}
