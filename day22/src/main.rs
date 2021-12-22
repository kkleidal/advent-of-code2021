use regex::Regex;
use std::cmp;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io;

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Coord {}

impl Hash for Coord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

#[derive(Debug)]
struct Cuboid {
    top_left: Coord,
    size: Coord,
}

struct CuboidIterator<'a> {
    cuboid: &'a Cuboid,
    current: Coord,
    exhausted: bool,
}

impl Cuboid {
    fn intersect(&self, other: &Self) -> Self {
        let xs = cmp::max(self.top_left.x, other.top_left.x);
        let ys = cmp::max(self.top_left.y, other.top_left.y);
        let zs = cmp::max(self.top_left.z, other.top_left.z);
        let xe = cmp::min(
            self.top_left.x + self.size.x,
            other.top_left.x + other.size.x,
        );
        let ye = cmp::min(
            self.top_left.y + self.size.y,
            other.top_left.y + other.size.y,
        );
        let ze = cmp::min(
            self.top_left.z + self.size.z,
            other.top_left.z + other.size.z,
        );
        Cuboid {
            top_left: Coord {
                x: xs,
                y: ys,
                z: zs,
            },
            size: Coord {
                x: cmp::max(0, xe - xs),
                y: cmp::max(0, ye - ys),
                z: cmp::max(0, ze - zs),
            },
        }
    }

    fn iter_coords(&self) -> CuboidIterator {
        CuboidIterator {
            cuboid: self,
            current: self.top_left,
            exhausted: false,
        }
    }
}

impl Iterator for CuboidIterator<'_> {
    type Item = Coord;

    fn next(&mut self) -> Option<Coord> {
        if self.exhausted {
            return None;
        }

        if self.cuboid.size.x == 0 || self.cuboid.size.y == 0 || self.cuboid.size.z == 0 {
            self.exhausted = true;
            return None;
        }

        let mut carry: i64 = 1;
        let next_x = if self.current.x + carry == self.cuboid.top_left.x + self.cuboid.size.x {
            carry = 1;
            self.cuboid.top_left.x
        } else {
            let out = self.current.x + carry;
            carry = 0;
            out
        };
        let next_y = if self.current.y + carry == self.cuboid.top_left.y + self.cuboid.size.y {
            carry = 1;
            self.cuboid.top_left.y
        } else {
            let out = self.current.y + carry;
            carry = 0;
            out
        };
        let next_z = if self.current.z + carry == self.cuboid.top_left.z + self.cuboid.size.z {
            carry = 1;
            self.cuboid.top_left.z
        } else {
            let out = self.current.z + carry;
            carry = 0;
            out
        };

        let out = Some(self.current.clone());

        if carry > 0 {
            self.exhausted = true;
            return out;
        }

        self.current.x = next_x;
        self.current.y = next_y;
        self.current.z = next_z;
        // println!("HERE: {:?} {:?}", self.current, self.cuboid);
        assert!(next_x < self.cuboid.top_left.x + self.cuboid.size.x);
        assert!(next_y < self.cuboid.top_left.y + self.cuboid.size.y);
        assert!(next_z < self.cuboid.top_left.z + self.cuboid.size.z);

        out
    }
}

#[derive(Debug)]
enum Instruction {
    ON(Cuboid),
    OFF(Cuboid),
}

impl Instruction {
    fn parse() -> Vec<Self> {
        // on x=-20..26,y=-36..17,z=-47..7
        //
        let re = Regex::new(r"^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$")
            .unwrap();
        let mut buffer = String::new();
        let mut out: Vec<Self> = Vec::new();
        loop {
            let n = io::stdin()
                .read_line(&mut buffer)
                .expect("Failed to read stdin");
            if n == 0 {
                // End of input
                break;
            }
            let cap = re.captures(buffer.trim()).expect("Couldn't parse line");
            let x_start: i64 = cap.get(2).unwrap().as_str().parse().unwrap();
            let x_end: i64 = cap.get(3).unwrap().as_str().parse().unwrap();
            let y_start: i64 = cap.get(4).unwrap().as_str().parse().unwrap();
            let y_end: i64 = cap.get(5).unwrap().as_str().parse().unwrap();
            let z_start: i64 = cap.get(6).unwrap().as_str().parse().unwrap();
            let z_end: i64 = cap.get(7).unwrap().as_str().parse().unwrap();
            let cuboid = Cuboid {
                top_left: Coord {
                    x: x_start,
                    y: y_start,
                    z: z_start,
                },
                size: Coord {
                    x: x_end - x_start + 1,
                    y: y_end - y_start + 1,
                    z: z_end - z_start + 1,
                },
            };
            out.push(match cap.get(1).unwrap().as_str() {
                "on" => Instruction::ON(cuboid),
                "off" => Instruction::OFF(cuboid),
                _ => panic!("Invalid instruction"),
            });
            buffer.clear();
        }
        out
    }
}

fn part1(instructions: &[Instruction]) -> usize {
    let mut currently_on: HashSet<Coord> = HashSet::new();
    let restriction = Cuboid {
        top_left: Coord {
            x: -50,
            y: -50,
            z: -50,
        },
        size: Coord {
            x: 101,
            y: 101,
            z: 101,
        },
    };
    for instruction in instructions {
        // println!("---");
        match instruction {
            Instruction::ON(cuboid) => {
                let limitted = cuboid.intersect(&restriction);
                for coord in limitted.iter_coords() {
                    if !currently_on.contains(&coord) {
                        // println!("ON {:?}", coord);
                        currently_on.insert(coord);
                    }
                }
            }
            Instruction::OFF(cuboid) => {
                let limitted = cuboid.intersect(&restriction);
                for coord in limitted.iter_coords() {
                    if currently_on.contains(&coord) {
                        // println!("OFF {:?}", coord);
                        currently_on.remove(&coord);
                    }
                }
            }
        }
    }
    currently_on.len()
}

fn main() {
    let instructions = Instruction::parse();
    println!("Hello, world! {:?}", instructions);
    println!("Part 1: {}", part1(&instructions[..]));
}
