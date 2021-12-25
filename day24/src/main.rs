use std::collections::{HashMap, LinkedList, HashSet};
use std::hash::{Hash, Hasher};
use std::fs::File;
use std::io::{self, BufRead};
use std::rc::{Rc, Weak};

#[derive(Debug, Clone, Copy)]
enum Operand {
    Constant(i64),
    Variable(char),
}

impl Operand {
    fn get_var(&self) -> char {
        match self {
            Operand::Constant(_) => panic!("Expected variable"),
            Operand::Variable(c) => *c,
        }
    }

    fn get_value(&self, state: &State) -> i64 {
        match self {
            Operand::Constant(c) => *c,
            Operand::Variable(c) => state.variable_store[c],
        }
    }

    fn parse(value: &str) -> Operand {
        match value.trim().parse::<i64>() {
            Err(_) => Operand::Variable(value.chars().nth(0).unwrap()),
            Ok(x) => Operand::Constant(x),
        }
    }
}

#[derive(Debug)]
enum ASTNode {
    Constant(i64),
    Inp(u8),
    Add(Rc<ASTNode>, Rc<ASTNode>),
    Mul(Rc<ASTNode>, Rc<ASTNode>),
    Div(Rc<ASTNode>, Rc<ASTNode>),
    Mod(Rc<ASTNode>, Rc<ASTNode>),
    Eql(Rc<ASTNode>, Rc<ASTNode>),
}


impl PartialEq for ASTNode {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ASTNode::Constant(x) => match other {
                ASTNode::Constant(y) => x == y,
                _ => false,
            },
            ASTNode::Inp(x) => match other {
                ASTNode::Inp(y) => x == y,
                _ => false,
            },
            ASTNode::Add(x1, y1) => match other {
                ASTNode::Add(x2, y2) => (x1 == x2 && y1 == y2),
                _ => false,
            },
            ASTNode::Mul(x1, y1) => match other {
                ASTNode::Mul(x2, y2) => (x1 == x2 && y1 == y2),
                _ => false,
            },
            ASTNode::Div(x1, y1) => match other {
                ASTNode::Div(x2, y2) => (x1 == x2 && y1 == y2),
                _ => false,
            },
            ASTNode::Mod(x1, y1) => match other {
                ASTNode::Mod(x2, y2) => (x1 == x2 && y1 == y2),
                _ => false,
            },
            ASTNode::Eql(x1, y1) => match other {
                ASTNode::Eql(x2, y2) => (x1 == x2 && y1 == y2),
                _ => false,
            },
        }
    }
}

impl Eq for ASTNode {}

impl Hash for ASTNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            ASTNode::Constant(x) => {
                0u8.hash(state);
                x.hash(state);
            },
            ASTNode::Inp(x) => {
                1u8.hash(state);
                x.hash(state);
            },
            ASTNode::Add(x1, y1) => {
                2u8.hash(state);
                x1.hash(state);
                y1.hash(state);
            },
            ASTNode::Mul(x1, y1) => {
                3u8.hash(state);
                x1.hash(state);
                y1.hash(state);
            },
            ASTNode::Div(x1, y1) => {
                4u8.hash(state);
                x1.hash(state);
                y1.hash(state);
            },
            ASTNode::Mod(x1, y1) => {
                5u8.hash(state);
                x1.hash(state);
                y1.hash(state);
            },
            ASTNode::Eql(x1, y1) => {
                6u8.hash(state);
                x1.hash(state);
                y1.hash(state);
            },
        }
    }
}

impl ASTNode {
    fn render(self: &Rc<Self>, indent: usize, depth_limit: usize) {
        if depth_limit == 0 {
            return
        }
        let prefix = (0..(2 * indent)).map(|_| " ").collect::<String>();
        match &**self {
            ASTNode::Constant(x) => {
                println!("{}- {}", prefix, x);
            },
            ASTNode::Inp(x) => {
                println!("{}- input[{}]", prefix, x);
            },
            ASTNode::Add(x, y) => {
                println!("{}- add:", prefix);
                ASTNode::render(x, indent + 1, depth_limit - 1);
                ASTNode::render(y, indent + 1, depth_limit - 1);
            },
            ASTNode::Mul(x, y) => {
                println!("{}- mul:", prefix);
                ASTNode::render(x, indent + 1, depth_limit - 1);
                ASTNode::render(y, indent + 1, depth_limit - 1);
            },
            ASTNode::Div(x, y) => {
                println!("{}- div:", prefix);
                ASTNode::render(x, indent + 1, depth_limit - 1);
                ASTNode::render(y, indent + 1, depth_limit - 1);
            },
            ASTNode::Mod(x, y) => {
                println!("{}- mod:", prefix);
                ASTNode::render(x, indent + 1, depth_limit - 1);
                ASTNode::render(y, indent + 1, depth_limit - 1);
            },
            ASTNode::Eql(x, y) => {
                println!("{}- eql:", prefix);
                ASTNode::render(x, indent + 1, depth_limit - 1);
                ASTNode::render(y, indent + 1, depth_limit - 1);
            },
        }
    }

    fn eval(self: &Rc<Self>, inputs: &[i64]) -> i64 {
        match &**self {
            ASTNode::Constant(x) => *x,
            ASTNode::Inp(x) => inputs[*x as usize],
            ASTNode::Add(x, y) => ASTNode::eval(x, inputs) + ASTNode::eval(y, inputs),
            ASTNode::Mul(x, y) => ASTNode::eval(x, inputs) * ASTNode::eval(y, inputs),
            ASTNode::Div(x, y) => ASTNode::eval(x, inputs) / ASTNode::eval(y, inputs),
            ASTNode::Mod(x, y) => ASTNode::eval(x, inputs) % ASTNode::eval(y, inputs),
            ASTNode::Eql(x, y) => if ASTNode::eval(x, inputs) == ASTNode::eval(y, inputs) { 1 } else { 0 },
        }
    }

    fn rops(self: &Rc<Self>, cache: &mut HashSet<Rc<ASTNode>>) -> usize {
        if cache.contains(self) {
            return 0;
        }
        cache.insert(Rc::clone(self));

        match &**self {
            ASTNode::Constant(_) => 0,
            ASTNode::Inp(_) => 0,
            ASTNode::Add(x, y) => ASTNode::rops(x, cache) + ASTNode::rops(y, cache) + 1,
            ASTNode::Mul(x, y) => ASTNode::rops(x, cache) + ASTNode::rops(y, cache) + 1,
            ASTNode::Div(x, y) => ASTNode::rops(x, cache) + ASTNode::rops(y, cache) + 1,
            ASTNode::Mod(x, y) => ASTNode::rops(x, cache) + ASTNode::rops(y, cache) + 1,
            ASTNode::Eql(x, y) => ASTNode::rops(x, cache) + ASTNode::rops(y, cache) + 1,

        }
    }
    fn ops(self: &Rc<Self>) -> usize {
        let mut cache: HashSet<Rc<ASTNode>> = HashSet::new();
        return ASTNode::rops(self, &mut cache);
    }

    fn r_inputs_used(self: &Rc<Self>, used: &mut HashSet<u8>) {
        match &**self {
            ASTNode::Constant(_) => (),
            ASTNode::Inp(x) => {
                used.insert(*x);
            },
            ASTNode::Add(x, y) => {
                ASTNode::r_inputs_used(x, used);
                ASTNode::r_inputs_used(y, used);
            },
            ASTNode::Mul(x, y) => {
                ASTNode::r_inputs_used(x, used);
                ASTNode::r_inputs_used(y, used);
            },
            ASTNode::Div(x, y) => {
                ASTNode::r_inputs_used(x, used);
                ASTNode::r_inputs_used(y, used);
            },
            ASTNode::Mod(x, y) => {
                ASTNode::r_inputs_used(x, used);
                ASTNode::r_inputs_used(y, used);
            },
            ASTNode::Eql(x, y) => {
                ASTNode::r_inputs_used(x, used);
                ASTNode::r_inputs_used(y, used);
            },
        }
    }

    fn inputs_used(self: &Rc<Self>) -> HashSet<u8> {
        let mut used: HashSet<u8> = HashSet::new();
        ASTNode::r_inputs_used(self, &mut used);
        used
    }

    fn rreplace_common(self: &Rc<Self>, cache: &mut HashSet<Rc<ASTNode>>) -> Rc<ASTNode> {
        if cache.contains(self) {
            return Rc::clone(cache.get(self).unwrap());
        }

        let common = match &**self {
            ASTNode::Constant(_) => Rc::clone(self),
            ASTNode::Inp(_) => Rc::clone(self),
            ASTNode::Add(x, y) => Rc::new(ASTNode::Add(ASTNode::rreplace_common(x, cache), ASTNode::rreplace_common(y, cache))),
            ASTNode::Mul(x, y) => Rc::new(ASTNode::Mul(ASTNode::rreplace_common(x, cache), ASTNode::rreplace_common(y, cache))),
            ASTNode::Div(x, y) => Rc::new(ASTNode::Div(ASTNode::rreplace_common(x, cache), ASTNode::rreplace_common(y, cache))),
            ASTNode::Mod(x, y) => Rc::new(ASTNode::Mod(ASTNode::rreplace_common(x, cache), ASTNode::rreplace_common(y, cache))),
            ASTNode::Eql(x, y) => Rc::new(ASTNode::Eql(ASTNode::rreplace_common(x, cache), ASTNode::rreplace_common(y, cache))),
        };

        if cache.contains(&common) {
            return Rc::clone(cache.get(&common).unwrap());
        }
        cache.insert(Rc::clone(&common));
        return common;
    }

    fn replace_common(self: &Rc<Self>) -> Rc<Self> {
        let mut cache: HashSet<Rc<ASTNode>> = HashSet::new();
        let out = ASTNode::rreplace_common(self, &mut cache);
        ASTNode::render(&out, 0, 25);

        out
    }

    // fn reval(self: &Rc<Self>, inputs: Vec<i64>, cache: HashMap<*const Self, i64>) -> i64 {
    //     let ptr = self.as_ptr();
    //     if cache.contains_key(&ptr) {
    //         return cache[&ptr]
    //     }
    //     // TODO: use the ptr as a cache key for the value of the sub tree. Recurse and evaluate.


    // }

    // fn eval(self: &Rc<Self>, inputs: Vec<i64>) -> i64 {
    //     return reval(self, inputs, HashMap::new());
    // }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Inp(Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Mod(Operand, Operand),
    Eql(Operand, Operand),
}

#[derive(Debug, Clone)]
struct State {
    input_queue: LinkedList<i64>,
    variable_store: HashMap<char, i64>,
    pc: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        if other.input_queue.len() != self.input_queue.len() {
            return false;
        }
        if other.variable_store.len() != self.variable_store.len() {
            return false;
        }
        if other.pc != self.pc {
            return false;
        }
        for (v1, v2) in self.input_queue.iter().zip(other.input_queue.iter()) {
            if v1 != v2 {
                return false;
            }
        }
        for (key, val) in self.variable_store.iter() {
            if !other.variable_store.contains_key(key) {
                return false;
            }
            if other.variable_store[key] != *val {
                return false;
            }
        }
        return true;
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.input_queue.len().hash(state);
        self.variable_store.len().hash(state);
        self.pc.hash(state);
        for v1 in self.input_queue.iter() {
            v1.hash(state);
        }
        let mut keys = self.variable_store.keys().cloned().collect::<Vec<char>>();
        keys.sort();
        for key in keys {
            key.hash(state);
            self.variable_store[&key].hash(state);
        }
    }
}

struct SmartComputer {
    program: Vec<Op>,
    cache: HashMap<State, bool>,
}

#[derive(Debug)]
struct Range {
    low: Option<i64>,
    high: Option<i64>,
}

impl Range {
    fn constant(&self) -> Option<i64> {
        let (low, high) = self.full_range()?;
        if low == high {
            Some(low)
        } else {
            None
        }
    }

    fn full_range(&self) -> Option<(i64,i64)> {
        let low = self.low?;
        let high = self.high?;
        return Some((low, high));
    }

    fn can_intersect(&self, other: &Self) -> bool {
        !(match self.low {
            None => match other.low {
                None => true,
                Some(their_low) => match self.high {
                    None => true,
                    Some(high_val) => their_low > high_val,
                },
            },
            Some(low_val) => match other.high {
                None => match self.high {
                    None => true,
                    Some(high_val) => match other.low {
                        None => true,
                        Some(their_low) => their_low > high_val,
                    }
                },
                Some(their_high) => their_high < low_val,
            },
        })
    }
}

impl SmartComputer {
    fn run(&mut self, input: usize, digits: usize) -> bool {
        let mut current = input;
        let mut inputs: Vec<i64> = vec![0; digits];
        for i in 0..digits {
            inputs[digits - 1 - i] = (current % 10) as i64;
            current /= 10;
        }

        let mut var_store: HashMap<char, i64> = HashMap::new();
        var_store.insert('w', 0);
        var_store.insert('x', 0);
        var_store.insert('y', 0);
        var_store.insert('z', 0);
        let mut states: Vec<State> = vec![State {
            input_queue: LinkedList::from_iter(inputs.iter().cloned()),
            variable_store: var_store,
            pc: 0,
        }];
        // println!("Input queue: {:?}", states[states.len() - 1].input_queue);
        // println!("Program: {:?}", self.program);
        while states[states.len() - 1].pc < self.program.len() && !self.cache.contains_key(&states[states.len() - 1]) {
            let mut state = states[states.len() - 1].clone();
            // println!("Op {}: {:?}", state.pc, self.program[state.pc]);
            match self.program[state.pc] {
                Op::Inp(var) => state.variable_store.insert(var.get_var(), match state.input_queue.pop_front() {
                    None => panic!("Requested input when exhausted!"),
                    Some(inp) => inp,
                }),
                Op::Add(lhs, rhs) => state.variable_store.insert(lhs.get_var(), lhs.get_value(&state) + rhs.get_value(&state)),
                Op::Mul(lhs, rhs) => state.variable_store.insert(lhs.get_var(), lhs.get_value(&state) * rhs.get_value(&state)),
                Op::Div(lhs, rhs) => state.variable_store.insert(lhs.get_var(), lhs.get_value(&state) / rhs.get_value(&state)),
                Op::Mod(lhs, rhs) => state.variable_store.insert(lhs.get_var(), lhs.get_value(&state) % rhs.get_value(&state)),
                Op::Eql(lhs, rhs) => state.variable_store.insert(lhs.get_var(), if lhs.get_value(&state) == rhs.get_value(&state) { 1 } else { 0 }),
            };
            state.pc += 1;
            states.push(state);
        }
        let res = if self.cache.contains_key(&states[states.len() - 1]) {
            self.cache[&states[states.len() - 1]]
        } else {
            let c = 'z';
            states[states.len() - 1].variable_store[&c] == 0
        };
        for state in states.iter().cloned() {
            self.cache.insert(state, res);
        }
        res
    }

    fn run_no_cache(&self, inputs: &[i64]) -> i64 {
        let mut var_store: HashMap<char, i64> = HashMap::new();
        var_store.insert('w', 0);
        var_store.insert('x', 0);
        var_store.insert('y', 0);
        var_store.insert('z', 0);
        let mut state = State {
            input_queue: LinkedList::from_iter(inputs.iter().cloned()),
            variable_store: var_store,
            pc: 0,
        };
        // println!("Input queue: {:?}", states[states.len() - 1].input_queue);
        // println!("Program: {:?}", self.program);
        while state.pc < self.program.len() {
            // println!("Op {}: {:?}", state.pc, self.program[state.pc]);
            match self.program[state.pc] {
                Op::Inp(var) => state.variable_store.insert(var.get_var(), match state.input_queue.pop_front() {
                    None => panic!("Requested input when exhausted!"),
                    Some(inp) => inp,
                }),
                Op::Add(lhs, rhs) => state.variable_store.insert(lhs.get_var(), lhs.get_value(&state) + rhs.get_value(&state)),
                Op::Mul(lhs, rhs) => state.variable_store.insert(lhs.get_var(), lhs.get_value(&state) * rhs.get_value(&state)),
                Op::Div(lhs, rhs) => state.variable_store.insert(lhs.get_var(), lhs.get_value(&state) / rhs.get_value(&state)),
                Op::Mod(lhs, rhs) => state.variable_store.insert(lhs.get_var(), lhs.get_value(&state) % rhs.get_value(&state)),
                Op::Eql(lhs, rhs) => state.variable_store.insert(lhs.get_var(), if lhs.get_value(&state) == rhs.get_value(&state) { 1 } else { 0 }),
            };
            state.pc += 1;
        }
        let c = 'z';
        state.variable_store[&c]
    }
    
    fn initialize(filename: &str) -> Self {
        let file = File::open(filename).unwrap();
        let reader = io::BufReader::new(file);
        let mut program: Vec<Op> = Vec::new();
        for line_res in reader.lines() {
            let line = line_res.unwrap();
            if line.trim().len() == 0 || line.chars().nth(0).unwrap() == '#' {
                continue;
            }
            let parts: Vec<&str> = line.split(" ").collect();
            let ops: Vec<Operand> = parts[1..].iter().map(|x| Operand::parse(x)).collect();
            program.push(match parts[0] {
                "inp" => Op::Inp(ops[0]),
                "add" => Op::Add(ops[0], ops[1]),
                "mul" => Op::Mul(ops[0], ops[1]),
                "div" => Op::Div(ops[0], ops[1]),
                "mod" => Op::Mod(ops[0], ops[1]),
                "eql" => Op::Eql(ops[0], ops[1]),
                _ => panic!("Invalid op: {}", parts[0]),
            });
        }
        Self {
            program,
            cache: HashMap::new(),
        }
    }

    fn resolve_ast(&self, var_store: &HashMap<char, Rc<ASTNode>>, op: Operand) -> Rc<ASTNode> {
        match op {
            Operand::Constant(x) => Rc::new(ASTNode::Constant(x)),
            Operand::Variable(c) => Rc::clone(&var_store[&c]),
        }
    }

    fn range(&self, tree: &Rc<ASTNode>) -> Range {
        match &**tree {
            ASTNode::Constant(x) => Range{
                low: Some(*x),
                high: Some(*x),
            },
            ASTNode::Inp(_) => Range{
                low: Some(1),
                high: Some(9),
            },
            ASTNode::Add(lhs, rhs) => {
                let lhs_range = self.range(lhs);
                let rhs_range = self.range(rhs);
                let low = match lhs_range.low {
                    None => None,
                    Some(lhs_low) => match rhs_range.low {
                        None => None,
                        Some(rhs_low) => Some(lhs_low + rhs_low),
                    }
                };
                let high = match lhs_range.high {
                    None => None,
                    Some(lhs_high) => match rhs_range.high {
                        None => None,
                        Some(rhs_high) => Some(lhs_high + rhs_high),
                    }
                };
                Range { low, high }
            },
            ASTNode::Mul(lhs, rhs) => {
                let lhs_range = self.range(lhs);
                let rhs_range = self.range(rhs);
                // Mul is complicated. Only handle cases where one is a constant
                match lhs_range.full_range() {
                    None => (),
                    Some((llow, lhigh)) => match rhs_range.full_range() {
                        None => (),
                        Some((rlow, rhigh)) => {
                            if llow >= 0 && lhigh >= 0 && rlow >= 0 && rhigh >= 0 {
                                return Range{
                                    low: Some(llow * rlow),
                                    high: Some(lhigh * rhigh),
                                };
                            }
                        }
                    }
                }
                match lhs_range.constant() {
                    None => (),
                    Some(c) => {
                        let low = match rhs_range.low {
                            None => None,
                            Some(low) => Some(low * c),
                        };
                        let high = match rhs_range.high {
                            None => None,
                            Some(high) => Some(high * c),
                        };
                        if c >= 0 {
                            return Range{ low, high }
                        } else {
                            return Range{ low: high, high: low }
                        }
                    }
                }
                match rhs_range.constant() {
                    None => (),
                    Some(c) => {
                        let low = match lhs_range.low {
                            None => None,
                            Some(low) => Some(low * c),
                        };
                        let high = match lhs_range.high {
                            None => None,
                            Some(high) => Some(high * c),
                        };
                        if c >= 0 {
                            return Range{ low, high }
                        } else {
                            return Range{ low: high, high: low }
                        }
                    }
                }
                Range { low: None, high: None }
            },
            ASTNode::Div(lhs, rhs) => {
                // Only handle case where denominator is constant
                let lhs_range = self.range(lhs);
                let rhs_range = self.range(rhs);
                match lhs_range.full_range() {
                    None => (),
                    Some((llow, lhigh)) => match rhs_range.full_range() {
                        None => (),
                        Some((rlow, rhigh)) => {
                            if llow >= 0 && lhigh >= 0 && rlow >= 0 && rhigh >= 0 {
                                return Range{
                                    low: Some(llow / rhigh),
                                    high: Some(lhigh / rlow),
                                };
                            }
                        }
                    }
                }
                match rhs_range.constant() {
                    None => (),
                    Some(c) => {
                        let low = match lhs_range.low {
                            None => None,
                            Some(low) => Some(low / c),
                        };
                        let high = match lhs_range.high {
                            None => None,
                            Some(high) => Some(high / c),
                        };
                        if c >= 0 {
                            return Range{ low, high }
                        } else {
                            return Range{ low: high, high: low }
                        }
                    }
                }
                Range { low: None, high: None }
            },
            ASTNode::Mod(lhs, rhs) => {
                let lhs_range = self.range(lhs);
                let rhs_range = self.range(rhs);
                match rhs_range.constant() {
                    None => (),
                    Some(modulo) => match lhs_range.low {
                        None => (),
                        Some(low) => match lhs_range.high {
                            None => (),
                            Some(high) => {
                                if high - low < modulo && high % modulo > low % modulo {
                                    return Range{low: Some(low % modulo), high: Some(high % modulo)}
                                }
                            }
                        }
                    }
                }
                Range{
                    low: Some(0),
                    high: rhs_range.high,
                }
            },
            ASTNode::Eql(lhs, rhs) => {
                Range{
                    low: Some(0),
                    high: Some(1),
                }
            },
        }
    }

    fn eval_constants(&self, tree: Rc<ASTNode>) -> Rc<ASTNode> {
        let recurse = false;
        match &*tree {
            ASTNode::Constant(_) => tree,
            ASTNode::Inp(_) => tree,
            ASTNode::Add(lhs, rhs) => {
                // let lhs_new = self.eval_constants(Rc::clone(lhs));
                // let rhs_new = self.eval_constants(Rc::clone(rhs));
                let lhs_new = Rc::clone(lhs);
                let rhs_new = Rc::clone(rhs);
                if let ASTNode::Constant(x) = &*lhs_new {
                    if *x == 0 {
                        return rhs_new;
                    }
                }
                if let ASTNode::Constant(x) = &*rhs_new {
                    if *x == 0 {
                        return lhs_new;
                    }
                    if let ASTNode::Constant(y) = &*lhs_new {
                        let out = Rc::new(ASTNode::Constant(*y + *x));
                        return out;
                    }
                }
                tree
                // Rc::new(ASTNode::Add(lhs_new, rhs_new))
            },
            ASTNode::Mul(lhs, rhs) => {
                // let lhs_new = self.eval_constants(Rc::clone(lhs));
                // let rhs_new = self.eval_constants(Rc::clone(rhs));
                let lhs_new = Rc::clone(lhs);
                let rhs_new = Rc::clone(rhs);
                if let ASTNode::Constant(x) = &*lhs_new {
                    if *x == 1 {
                        return rhs_new;
                    }
                    if *x == 0 {
                        return lhs_new;
                    }
                }
                if let ASTNode::Constant(x) = &*rhs_new {
                    if *x == 1 {
                        return lhs_new;
                    }
                    if *x == 0 {
                        return rhs_new;
                    }
                    if let ASTNode::Constant(y) = &*lhs_new {
                        let out = Rc::new(ASTNode::Constant(*y * *x));
                        return out;
                    }
                }
                tree
                // Rc::new(ASTNode::Mul(lhs_new, rhs_new))
            },
            ASTNode::Div(lhs, rhs) => {
                // let lhs_new = self.eval_constants(Rc::clone(lhs));
                // let rhs_new = self.eval_constants(Rc::clone(rhs));
                let lhs_new = Rc::clone(lhs);
                let rhs_new = Rc::clone(rhs);
                if let ASTNode::Constant(x) = &*lhs_new {
                    if *x == 0 {
                        return lhs_new;
                    }
                }
                if let ASTNode::Constant(x) = &*rhs_new {
                    if *x == 1 {
                        return lhs_new;
                    }
                    if let ASTNode::Constant(y) = &*lhs_new {
                        let out = Rc::new(ASTNode::Constant(*y / *x));
                        return out;
                    }
                }
                tree
                // Rc::new(ASTNode::Div(lhs_new, rhs_new))
            },
            ASTNode::Mod(lhs, rhs) => {
                // let lhs_new = self.eval_constants(Rc::clone(lhs));
                // let rhs_new = self.eval_constants(Rc::clone(rhs));
                let lhs_new = Rc::clone(lhs);
                let rhs_new = Rc::clone(rhs);
                if let ASTNode::Constant(x) = &*rhs_new {
                    if *x == 1 {
                        let out = Rc::new(ASTNode::Constant(0));
                        return out;
                    }
                    if let ASTNode::Constant(y) = &*lhs_new {
                        let out = Rc::new(ASTNode::Constant(*y % *x));
                        return out;
                    }
                }
                tree
                // Rc::new(ASTNode::Mod(lhs_new, rhs_new))
            },
            ASTNode::Eql(lhs, rhs) => {
                // let lhs_new = self.eval_constants(Rc::clone(lhs));
                // let rhs_new = self.eval_constants(Rc::clone(rhs));
                let lhs_new = Rc::clone(lhs);
                let rhs_new = Rc::clone(rhs);
                if let ASTNode::Constant(x) = &*rhs_new {
                    if let ASTNode::Constant(y) = &*lhs_new {
                        let out = Rc::new(ASTNode::Constant(if *y == *x { 1 } else { 0 }));
                        return out;
                    }
                }
                let lhs_range = self.range(&lhs_new);
                let rhs_range = self.range(&rhs_new);
                if !lhs_range.can_intersect(&rhs_range) {
                    println!("{:?} can't intersect {:?}", lhs_range, rhs_range);
                    let out = Rc::new(ASTNode::Constant(0));
                    return out;
                }
                tree
                // Rc::new(ASTNode::Eql(lhs_new, rhs_new))
            },
        }
    }

    fn simplify(&self) -> Rc<ASTNode> {
        let mut var_store: HashMap<char, Rc<ASTNode>> = HashMap::new();
        let mut inp_number: u8 = 0;
        let zero = Rc::new(ASTNode::Constant(0));
        var_store.insert('w', Rc::clone(&zero));
        var_store.insert('x', Rc::clone(&zero));
        var_store.insert('y', Rc::clone(&zero));
        var_store.insert('z', Rc::clone(&zero));
        for op in self.program.iter() {
            println!(":: {:?}", op);
            let var = match op {
                Op::Inp(var) => var.get_var(),
                Op::Add(lhs, rhs) => lhs.get_var(),
                Op::Mul(lhs, rhs) => lhs.get_var(),
                Op::Div(lhs, rhs) => lhs.get_var(),
                Op::Mod(lhs, rhs) => lhs.get_var(),
                Op::Eql(lhs, rhs) => lhs.get_var(),
            };
            match op {
                Op::Inp(var) => {
                    var_store.insert(var.get_var(), Rc::new(ASTNode::Inp(inp_number)));
                    inp_number += 1;
                },
                Op::Add(lhs, rhs) => {
                    var_store.insert(lhs.get_var(), self.eval_constants(Rc::new(ASTNode::Add(
                            self.resolve_ast(&var_store, *lhs),
                            self.resolve_ast(&var_store, *rhs),
                    ))));
                },
                Op::Mul(lhs, rhs) => {
                    var_store.insert(lhs.get_var(), self.eval_constants(Rc::new(ASTNode::Mul(
                            self.resolve_ast(&var_store, *lhs),
                            self.resolve_ast(&var_store, *rhs),
                    ))));
                },
                Op::Div(lhs, rhs) => {
                    var_store.insert(lhs.get_var(), self.eval_constants(Rc::new(ASTNode::Div(
                            self.resolve_ast(&var_store, *lhs),
                            self.resolve_ast(&var_store, *rhs),
                    ))));
                },
                Op::Mod(lhs, rhs) => {
                    var_store.insert(lhs.get_var(), self.eval_constants(Rc::new(ASTNode::Mod(
                            self.resolve_ast(&var_store, *lhs),
                            self.resolve_ast(&var_store, *rhs),
                    ))));
                },
                Op::Eql(lhs, rhs) => {
                    var_store.insert(lhs.get_var(), self.eval_constants(Rc::new(ASTNode::Eql(
                            self.resolve_ast(&var_store, *lhs),
                            self.resolve_ast(&var_store, *rhs),
                    ))));
                },
            };
            println!("  variable {} is:", var);
            ASTNode::render(&var_store[&var], 0, 5);
            println!("  with range {:?}:", self.range(&var_store[&var]));
            println!();
        }
        let mut ast = var_store[&'z'].clone();
        ast = self.eval_constants(ast);
        // ast = ASTNode::replace_common(&ast);
        ASTNode::render(&ast, 0, 15);
        println!("Ops: {}", ASTNode::ops(&ast));
        println!("Inputs used: {:?}", ASTNode::inputs_used(&ast));
        ast
        // println!("AST: {:?}", ast);
    }
}

fn digits_to_number(digits: &[i64]) -> i64 {
    let mut out: i64 = 0;
    for digit in digits {
        out = (out * 10) + digit;
    }
    out
}

fn main() {
    println!("Hello, world!");
    let mut computer = crate::SmartComputer::initialize("mine.txt");
    let ast = computer.simplify();

    let mut found: Option<i64> = None;
    let highest = 96299896449997_i64;
    let lowest = 31162141116841_i64;
    let v = lowest;
    // for v in (1000000..10000000).rev() {
    if v % 1000 == 0 {
        println!("{}% finished", ((10000000 - v) as f64) / ((10000000 - 1000000) as f64) * 100.0);
    }
    let mut inputs: Vec<i64> = vec![9; 14];
    for i in 0..14 {
        inputs[i] = (v / 10_i64.pow(13_u32 - i as u32)) % 10;
    }
    println!("HERE: {:?}", inputs);
    // if inputs.iter().any(|x| *x == 0) {
    //     continue;
    // }
    let z1 = ASTNode::eval(&ast, &inputs[..]);
    let z2 = computer.run_no_cache(&inputs[..]);
    println!("HERE {} {}", z1, z2);
    //assert_eq!(z1, z2);
    if z1 == 0 {
        found = Some(digits_to_number(&inputs[..]));
        // break;
    }
    // }
    println!("Part 1: {:?}", found);
}

#[cfg(test)]
mod tests {
    #[test]
    fn factor_of_3() {
        let mut computer = crate::SmartComputer::initialize("example.txt");
        assert_eq!(false, computer.run(39, 2));
        // With cache:
        assert_eq!(false, computer.run(39, 2));
        assert_eq!(true, computer.run(38, 2));
        assert_eq!(false, computer.run(26, 2));
        assert_eq!(false, computer.run(13, 2));
        assert_eq!(true, computer.run(12, 2));
        assert_eq!(false, computer.run(00, 2));
    }

    #[test]
    fn binary() {
        let mut computer = crate::SmartComputer::initialize("example2.txt");
        assert_eq!(false, computer.run(9, 1));
        assert_eq!(true, computer.run(8, 1));
        assert_eq!(false, computer.run(7, 1));
        assert_eq!(true, computer.run(6, 1));
    }
}

