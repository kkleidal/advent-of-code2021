use regex::Regex;
use std::cmp;
use std::hash::{Hash, Hasher};
use std::io;
use std::ops::{Add, BitAnd};

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
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

#[derive(Debug, Clone, Copy)]
struct Cuboid {
    top_left: Coord,
    bottom_right: Coord,
    size: Coord,
}

impl PartialEq for Cuboid {
    fn eq(&self, other: &Self) -> bool {
        self.top_left == other.top_left && self.size == other.size
    }
}

impl Eq for Cuboid {}

impl Hash for Cuboid {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.top_left.hash(state);
        self.size.hash(state);
    }
}

impl Cuboid {
    fn new(top_left: Coord, size: Coord) -> Self {
        if size.x == 0 || size.y == 0 || size.z == 0 {
            let origin = Coord { x: 0, y: 0, z: 0 };
            return Self {
                top_left: origin,
                size: origin,
                bottom_right: origin,
            };
        } else {
            return Self {
                top_left: top_left,
                size: size,
                bottom_right: top_left + size,
            };
        }
    }

    fn is_empty(&self) -> bool {
        self.size.x == 0 || self.size.y == 0 || self.size.z == 0
    }

    fn area(&self) -> u64 {
        let out: u64 = (self.size.x * self.size.y * self.size.z)
            .try_into()
            .expect("Invalid u64");
        out
    }

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
        Cuboid::new(
            Coord {
                x: xs,
                y: ys,
                z: zs,
            },
            Coord {
                x: cmp::max(0, xe - xs),
                y: cmp::max(0, ye - ys),
                z: cmp::max(0, ze - zs),
            },
        )
    }

    fn difference(&self, other: &Self) -> Vec<Self> {
        let mid = self.intersect(other);

        if mid.is_empty() {
            return vec![self.clone()];
        }

        let mut out: Vec<Self> = Vec::new();

        if mid.top_left.x > self.top_left.x {
            // We need to add the cuboid between the other top left x and self's top left x
            out.push(Cuboid::new(
                Coord {
                    x: self.top_left.x,
                    y: self.top_left.y,
                    z: self.top_left.z,
                },
                Coord {
                    x: mid.top_left.x - self.top_left.x,
                    y: self.size.y,
                    z: self.size.z,
                },
            ));
        }
        if self.bottom_right.x > mid.bottom_right.x {
            // We need to add the cuboid between the common right x and other right x
            out.push(Cuboid::new(
                Coord {
                    x: mid.bottom_right.x,
                    y: self.top_left.y,
                    z: self.top_left.z,
                },
                Coord {
                    x: self.bottom_right.x - mid.bottom_right.x,
                    y: self.size.y,
                    z: self.size.z,
                },
            ));
        }

        // So now we're only concerned with the regions between mid.top_left.x and
        // mid.bottom_right.x
        if mid.top_left.y > self.top_left.y {
            out.push(Cuboid::new(
                Coord {
                    x: mid.top_left.x,
                    y: self.top_left.y,
                    z: self.top_left.z,
                },
                Coord {
                    x: mid.size.x,
                    y: mid.top_left.y - self.top_left.y,
                    z: self.size.z,
                },
            ));
        }
        if self.bottom_right.y > mid.bottom_right.y {
            out.push(Cuboid::new(
                Coord {
                    x: mid.top_left.x,
                    y: mid.bottom_right.y,
                    z: self.top_left.z,
                },
                Coord {
                    x: mid.size.x,
                    y: self.bottom_right.y - mid.bottom_right.y,
                    z: self.size.z,
                },
            ));
        }

        // So now we're only concerned with the regions between mid.top_left.x and
        // mid.bottom_right.x and mid.top_left.y and mid.bottom_right.y
        if mid.top_left.z > self.top_left.z {
            out.push(Cuboid::new(
                Coord {
                    x: mid.top_left.x,
                    y: mid.top_left.y,
                    z: self.top_left.z,
                },
                Coord {
                    x: mid.size.x,
                    y: mid.size.y,
                    z: mid.top_left.z - self.top_left.z,
                },
            ));
        }
        if self.bottom_right.z > mid.bottom_right.z {
            out.push(Cuboid::new(
                Coord {
                    x: mid.top_left.x,
                    y: mid.top_left.y,
                    z: mid.bottom_right.z,
                },
                Coord {
                    x: mid.size.x,
                    y: mid.size.y,
                    z: self.bottom_right.z - mid.bottom_right.z,
                },
            ));
        }

        for cube in out.iter() {
            assert!(!cube.is_empty());
            assert!(cube.intersect(self) == *cube);
            assert!(cube.intersect(other).is_empty());
        }
        for i in 0..out.len() {
            for j in (i + 1)..out.len() {
                // Mutually exclusive
                assert!(out[i].intersect(&out[j]).is_empty());
            }
        }
        out
    }
}

impl BitAnd for Cuboid {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a & b`
    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersect(&rhs)
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
            let cuboid = Cuboid::new(
                Coord {
                    x: x_start,
                    y: y_start,
                    z: z_start,
                },
                Coord {
                    x: x_end - x_start + 1,
                    y: y_end - y_start + 1,
                    z: z_end - z_start + 1,
                },
            );
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

fn count_cubes(instructions: &[Instruction], restriction: Option<Cuboid>) -> u64 {
    let mut on_cubes: Vec<Cuboid> = Vec::new();
    for instruction in instructions {
        match instruction {
            Instruction::ON(cuboid) => {
                let limitted = match restriction {
                    Some(bounds) => cuboid.intersect(&bounds),
                    None => cuboid.clone(),
                };
                let mut new_on_cubes: Vec<Cuboid> = vec![limitted];
                for cube in on_cubes {
                    let parts = cube.difference(&limitted);
                    new_on_cubes.extend_from_slice(&parts[..]);
                }
                on_cubes = new_on_cubes;
            }
            Instruction::OFF(cuboid) => {
                let limitted = match restriction {
                    Some(bounds) => cuboid.intersect(&bounds),
                    None => cuboid.clone(),
                };
                let mut new_on_cubes: Vec<Cuboid> = Vec::new();
                for cube in on_cubes {
                    let parts = cube.difference(&limitted);
                    new_on_cubes.extend_from_slice(&parts[..]);
                }
                on_cubes = new_on_cubes;
            }
        }
    }
    on_cubes.iter().map(|x| x.area()).sum()
}

fn main() {
    let instructions = Instruction::parse();
    let part1_restriction = Cuboid::new(
        Coord {
            x: -50,
            y: -50,
            z: -50,
        },
        Coord {
            x: 101,
            y: 101,
            z: 101,
        },
    );
    println!(
        "Part 1: {}",
        count_cubes(&instructions[..], Some(part1_restriction))
    );
    println!("Part 2: {}", count_cubes(&instructions[..], None));
}
