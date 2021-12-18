use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

struct Arena {
    nodes: Vec<Node>,
}

#[derive(Clone, Copy)]
struct NodeIndex {
    index: usize,
}

impl PartialEq for NodeIndex {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

struct Pair {
    left: NodeIndex,
    right: NodeIndex,
}

enum NodeKind {
    Pair(Pair),
    Regular(i64),
}

struct Node {
    parent: Option<NodeIndex>,
    kind: NodeKind,
}

impl Arena {
    fn new() -> Self {
        Arena { nodes: Vec::new() }
    }

    fn parse(&mut self, value: &str) -> NodeIndex {
        let (node, n) = self._parse_inner(None, value);
        if n != value.len() {
            panic!("Parse error");
        }
        node
    }

    fn alloc(&mut self) -> NodeIndex {
        self.nodes.push(Node {
            parent: None,
            kind: NodeKind::Regular(0),
        });
        NodeIndex {
            index: self.nodes.len() - 1,
        }
    }

    fn format(&self, index: NodeIndex) -> String {
        let node = self.deref(&index);
        match &node.kind {
            NodeKind::Pair(pair) => {
                format!("[{},{}]", self.format(pair.left), self.format(pair.right))
            }
            NodeKind::Regular(x) => {
                format!("{}", x)
            }
        }
    }

    fn deref(&self, index: &NodeIndex) -> &Node {
        &self.nodes[index.index]
    }

    fn deref_mut(&mut self, index: &NodeIndex) -> &mut Node {
        &mut self.nodes[index.index]
    }

    fn add(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
        let my_node_index = self.alloc();

        let mut lhs_node = self.deref_mut(&lhs);
        lhs_node.parent = Some(my_node_index);

        let mut rhs_node = self.deref_mut(&rhs);
        rhs_node.parent = Some(my_node_index);

        let mut my_node = self.deref_mut(&my_node_index);
        my_node.parent = None;
        my_node.kind = NodeKind::Pair(Pair {
            left: lhs,
            right: rhs,
        });
        my_node_index
    }

    fn _parse_inner(&mut self, parent: Option<NodeIndex>, value: &str) -> (NodeIndex, usize) {
        let c = value.chars().nth(0).unwrap();
        match c {
            '[' => {
                let my_node_index = self.alloc();

                let (left, consumed) = self._parse_inner(Some(my_node_index), &value[1..]);
                let pos = 1 + consumed;
                assert_eq!(value.chars().nth(pos).unwrap(), ',');
                let pos = pos + 1;
                let (right, consumed) = self._parse_inner(Some(my_node_index), &value[pos..]);
                let pos = pos + consumed;
                assert_eq!(value.chars().nth(pos).unwrap(), ']');
                let pos = pos + 1;

                let mut my_node = self.deref_mut(&my_node_index);
                my_node.parent = parent;
                my_node.kind = NodeKind::Pair(Pair { left, right });
                (my_node_index, pos)
            }
            _ => {
                let mut consumed = 0;
                let re = Regex::new(r"^\d$").unwrap();
                let mut out_value: i64 = 0;
                for c in value.chars() {
                    let cstr = c.to_string();
                    if re.is_match(&cstr[..]) {
                        let x: i64 = cstr.parse().unwrap();
                        out_value = out_value * 10 + x;
                        consumed += 1;
                    } else {
                        break;
                    }
                }
                let my_node_index = self.alloc();
                let mut my_node = self.deref_mut(&my_node_index);
                my_node.parent = parent;
                my_node.kind = NodeKind::Regular(out_value);
                (my_node_index, consumed)
            }
        }
    }

    fn depth(&self, node: NodeIndex) -> usize {
        let mut depth = 0;
        let mut current = node;
        loop {
            let cur_node = self.deref(&current);
            match cur_node.parent {
                None => break,
                Some(pid) => {
                    current = pid;
                    depth += 1;
                }
            }
        }
        depth
    }

    fn root(&self, node: NodeIndex) -> NodeIndex {
        let mut current = node;
        loop {
            let cur_node = self.deref(&current);
            match cur_node.parent {
                None => break,
                Some(parent) => {
                    current = parent;
                }
            }
        }
        current
    }

    fn parent(&self, node: NodeIndex) -> Option<NodeIndex> {
        let current = self.deref(&node);
        current.parent
    }

    fn unwrap_value(&self, node: NodeIndex) -> i64 {
        match self.deref(&node).kind {
            NodeKind::Regular(x) => x,
            NodeKind::Pair(_) => panic!("Unexpected pair"),
        }
    }

    fn try_explode(&mut self, node: NodeIndex) -> bool {
        if self.depth(node) == 5 {
            let parent_id = self.parent(node).unwrap();
            let explode_node = self.deref(&parent_id);
            if let NodeKind::Pair(pair) = &explode_node.kind {
                let left_val = self.unwrap_value(pair.left);
                let right_val = self.unwrap_value(pair.right);
                let left_id = pair.left;
                let right_id = pair.right;

                match self.left(left_id) {
                    None => (),
                    Some(left_id) => {
                        let current_val = self.unwrap_value(left_id);
                        let mut left_node = self.deref_mut(&left_id);
                        left_node.kind = NodeKind::Regular(left_val + current_val);
                    }
                }
                match self.right(right_id) {
                    None => (),
                    Some(right_id) => {
                        let current_val = self.unwrap_value(right_id);
                        let mut right_node = self.deref_mut(&right_id);
                        right_node.kind = NodeKind::Regular(right_val + current_val);
                    }
                }
            } else {
                panic!("Unexpected leaf node");
            }
            let mut explode_node = self.deref_mut(&parent_id);
            explode_node.kind = NodeKind::Regular(0);
            return true;
        } else {
            return false;
        }
    }

    fn try_split(&mut self, node: NodeIndex) -> bool {
        let current_val = self.unwrap_value(node);
        if current_val >= 10 {
            let left_id = self.alloc();
            let right_id = self.alloc();

            let mut left_node = self.deref_mut(&left_id);
            left_node.parent = Some(node);
            left_node.kind = NodeKind::Regular(current_val / 2);

            let mut right_node = self.deref_mut(&right_id);
            right_node.parent = Some(node);
            right_node.kind = NodeKind::Regular(current_val - current_val / 2);

            let mut split_node = self.deref_mut(&node);
            split_node.kind = NodeKind::Pair(Pair {
                left: left_id,
                right: right_id,
            });
            return true;
        } else {
            return false;
        }
    }

    fn reduce_once(&mut self, node: NodeIndex) -> bool {
        let root = self.root(node);
        let mut current = self.left_most(root);
        loop {
            if self.try_explode(current) {
                return true;
            }
            match self.right(current) {
                None => break,
                Some(next) => {
                    current = next;
                }
            }
        }
        let mut current = self.left_most(root);
        loop {
            if self.try_split(current) {
                return true;
            }
            match self.right(current) {
                None => break,
                Some(next) => {
                    current = next;
                }
            }
        }
        return false;
    }

    fn reduce(&mut self, node: NodeIndex) {
        loop {
            if !self.reduce_once(node) {
                break;
            }
        }
    }

    fn left_most(&self, node: NodeIndex) -> NodeIndex {
        let mut current = node;
        loop {
            let cur_node = self.deref(&current);
            match &cur_node.kind {
                NodeKind::Regular(_) => break,
                NodeKind::Pair(pair) => {
                    current = pair.left;
                }
            }
        }
        current
    }

    fn right_most(&self, node: NodeIndex) -> NodeIndex {
        let mut current = node;
        loop {
            let cur_node = self.deref(&current);
            match &cur_node.kind {
                NodeKind::Regular(_) => break,
                NodeKind::Pair(pair) => {
                    current = pair.right;
                }
            }
        }
        current
    }

    fn right(&self, node: NodeIndex) -> Option<NodeIndex> {
        {
            let cur_node = self.deref(&node);
            match &cur_node.kind {
                NodeKind::Regular(_) => (),
                NodeKind::Pair(_) => panic!("Cannot call right on a branch node"),
            }
        }
        // Go up to the branch until we're not the right node any more
        let mut current = node;
        loop {
            let cur_node = self.deref(&current);
            match cur_node.parent {
                None => {
                    // Right most node
                    return None;
                }
                Some(parent_id) => {
                    let parent_node = self.deref(&parent_id);
                    if let NodeKind::Pair(pair) = &parent_node.kind {
                        if pair.right == current {
                            current = parent_id;
                        } else {
                            // Otherwise, we're the left node, and we can start going down to the right
                            current = pair.right;
                            break;
                        }
                    } else {
                        panic!("Non branch node encountered as parent");
                    }
                }
            }
        }
        Some(self.left_most(current))
    }

    fn left(&self, node: NodeIndex) -> Option<NodeIndex> {
        {
            let cur_node = self.deref(&node);
            match &cur_node.kind {
                NodeKind::Regular(_) => (),
                NodeKind::Pair(_) => panic!("Cannot call right on a branch node"),
            }
        }
        // Go up to the branch until we're not the left node any more
        let mut current = node;
        loop {
            let cur_node = self.deref(&current);
            match cur_node.parent {
                None => {
                    // Left most node
                    return None;
                }
                Some(parent_id) => {
                    let parent_node = self.deref(&parent_id);
                    if let NodeKind::Pair(pair) = &parent_node.kind {
                        if pair.left == current {
                            current = parent_id;
                        } else {
                            // Otherwise, we're the right node, and we can start going down to the left
                            current = pair.left;
                            break;
                        }
                    } else {
                        panic!("Non branch node encountered as parent");
                    }
                }
            }
        }
        Some(self.right_most(current))
    }

    fn flatten(&self, root: NodeIndex) -> Vec<i64> {
        let mut current = Some(self.left_most(root));
        let mut out: Vec<i64> = Vec::new();
        loop {
            match current {
                None => break,
                Some(current_id) => {
                    let cur_node = self.deref(&current_id);
                    if let NodeKind::Regular(x) = cur_node.kind {
                        out.push(x);
                    }
                    current = self.right(current_id);
                }
            }
        }
        out
    }

    fn flatten_rev(&self, root: NodeIndex) -> Vec<i64> {
        let mut current = Some(self.right_most(root));
        let mut out: Vec<i64> = Vec::new();
        loop {
            match current {
                None => break,
                Some(current_id) => {
                    let cur_node = self.deref(&current_id);
                    if let NodeKind::Regular(x) = cur_node.kind {
                        out.push(x);
                    }
                    current = self.left(current_id);
                }
            }
        }
        out
    }

    fn load_file(&mut self, file: &mut File) -> Option<NodeIndex> {
        let stream = io::BufReader::new(file);
        let mut current: Option<NodeIndex> = None;
        for line in stream.lines() {
            if let Ok(line_str) = line {
                let next = self.parse((&line_str[..]).trim());
                current = match current {
                    None => Some(next),
                    Some(current_node) => Some(self.add(current_node, next)),
                };
                self.reduce(current.unwrap());
            }
        }
        current
    }

    fn magnitude(&self, node: NodeIndex) -> i64 {
        let cur_node = self.deref(&node);
        match &cur_node.kind {
            NodeKind::Regular(x) => *x,
            NodeKind::Pair(pair) => 3 * self.magnitude(pair.left) + 2 * self.magnitude(pair.right),
        }
    }

    fn load_file_vec(&mut self, file: &mut File) -> Vec<String> {
        let stream = io::BufReader::new(file);
        let mut out: Vec<String> = Vec::new();
        for line in stream.lines() {
            if let Ok(line_str) = line {
                out.push(line_str.trim().to_string());
            }
        }
        out
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let mut arena = Arena::new();

    let mut file = File::open(filename).unwrap();
    let node = arena.load_file(&mut file).unwrap();

    println!("Part 1: {}", arena.magnitude(node));

    let mut file = File::open(filename).unwrap();
    let nodes = arena.load_file_vec(&mut file);

    let mut possible: Vec<i64> = Vec::new();
    for i in 0..nodes.len() {
        for j in 0..nodes.len() {
            if i == j {
                continue;
            }
            let mut my_arena = Arena::new();
            let node1 = my_arena.parse(&nodes[i][..]);
            let node2 = my_arena.parse(&nodes[j][..]);
            let sum = my_arena.add(node1, node2);
            my_arena.reduce(sum);
            let mag = my_arena.magnitude(sum);
            possible.push(my_arena.magnitude(sum));
        }
    }
    println!("Part 2: {}", possible.iter().max().unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn parses_and_formats() {
        let mut arena = crate::Arena::new();
        let inp = "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";
        let node = arena.parse(inp);
        assert_eq!(format!("{}", arena.format(node)), inp);
    }

    #[test]
    fn adds() {
        let mut arena = crate::Arena::new();
        let node = arena.parse("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
        let node2 = arena.parse("[4,3]");
        let node_out = arena.add(node, node2);
        assert_eq!(
            format!("{}", arena.format(node_out)),
            "[[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]],[4,3]]"
        );
    }

    #[test]
    fn left_most_right_most() {
        let mut arena = crate::Arena::new();
        let node = arena.parse("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
        let left_most = arena.left_most(node);
        let right_most = arena.right_most(node);
        assert_eq!(format!("{}", arena.format(left_most)), "1");
        assert_eq!(format!("{}", arena.format(right_most)), "3");
    }

    #[test]
    fn flatten_right() {
        let mut arena = crate::Arena::new();
        let node = arena.parse("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
        let flat = arena.flatten(node);
        assert_eq!(
            format!("{:?}", flat),
            "[1, 3, 5, 3, 1, 3, 8, 7, 4, 9, 6, 9, 8, 2, 7, 3]"
        );
    }

    #[test]
    fn flatten_left() {
        let mut arena = crate::Arena::new();
        let node = arena.parse("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
        let flat = arena.flatten_rev(node);
        assert_eq!(
            format!("{:?}", flat),
            "[3, 7, 2, 8, 9, 6, 9, 4, 7, 8, 3, 1, 3, 5, 3, 1]"
        );
    }

    #[test]
    fn reduce1() {
        let mut arena = crate::Arena::new();
        let node1 = arena.parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let node2 = arena.parse("[1,1]");
        let node3 = arena.add(node1, node2);
        arena.reduce(node3);
        assert_eq!(arena.format(node3), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    macro_rules! one_reduction_test {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (inp, expected) = $value;
                let mut arena = crate::Arena::new();
                let node = arena.parse(inp);
                let reduced = arena.reduce_once(node);
                assert_eq!(reduced, true);
                assert_eq!(arena.format(node), expected);
            }
        )*
        }
    }

    one_reduction_test! {
        explode1: ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
        explode2: ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
        explode3: ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
        explode4: ("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
        explode5: ("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"),
    }

    macro_rules! load_and_reduce_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (filename, expected) = $value;
                let mut arena = crate::Arena::new();

                let mut file = crate::File::open(filename).unwrap();
                let node = arena.load_file(&mut file).unwrap();

                assert_eq!(arena.format(node), expected);
            }
        )*
        }
    }

    load_and_reduce_tests! {
        example1: ("example1.txt", "[[[[1,1],[2,2]],[3,3]],[4,4]]"),
        example2: ("example2.txt", "[[[[3,0],[5,3]],[4,4]],[5,5]]"),
        example3: ("example3.txt", "[[[[5,0],[7,4]],[5,5]],[6,6]]"),
        example4: ("example4.txt", "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"),
        example5: ("example5.txt", "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"),
    }

    macro_rules! mag_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (inp, expected) = $value;
                let mut arena = crate::Arena::new();
                let node = arena.parse(inp);
                assert_eq!(arena.magnitude(node), expected);
            }
        )*
        }
    }

    mag_tests! {
        mag1: ("[[1,2],[[3,4],5]]", 143),
        mag2: ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
        mag3: ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
        mag4: ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
        mag5: ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
        mag6: ("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488),
    }
}
