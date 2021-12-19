use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(point: Vec<i64>) -> Self {
        assert_eq!(point.len(), 3);
        Self {
            x: point[0],
            y: point[1],
            z: point[2],
        }
    }

    fn transform(&self, rot: (i64, i64, i64, i64, i64, i64, i64, i64, i64)) -> Point {
        Point {
            x: rot.0 * self.x + rot.1 * self.y + rot.2 * self.z,
            y: rot.3 * self.x + rot.4 * self.y + rot.5 * self.z,
            z: rot.6 * self.x + rot.7 * self.y + rot.8 * self.z,
        }
    }

    fn manhattan(&self, other: &Self) -> i64 {
        let vec = *self - *other;
        vec.x.abs() + vec.y.abs() + vec.z.abs()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

struct Mapping {
    rotation: (i64, i64, i64, i64, i64, i64, i64, i64, i64),
    offset_vector: Point,
}

impl Mapping {
    fn new(rotation: (i64, i64, i64, i64, i64, i64, i64, i64, i64), offset: Point) -> Self {
        Self {
            rotation,
            offset_vector: offset,
        }
    }

    fn identity() -> Self {
        Self {
            rotation: (1, 0, 0, 0, 1, 0, 0, 0, 1),
            offset_vector: Point { x: 0, y: 0, z: 0 },
        }
    }

    // fn rotate(&self, vector: Point) -> Point {
    //     vector.transform(self.rotation)
    // }
    //
    // fn transform(&self, point: Point) -> Point {
    //     point.transform(self.rotation) + self.offset_vector
    // }

    fn compose(&self, other: &Mapping) -> Mapping {
        let new_offset = self.offset_vector.transform(other.rotation) + other.offset_vector;
        let new_rotation = mm(self.rotation, other.rotation);
        Mapping {
            rotation: new_rotation,
            offset_vector: new_offset,
        }
    }
}

fn parse() -> Vec<Vec<Point>> {
    let mut scanners: Vec<Vec<Point>> = Vec::new();
    let mut current_scanner: Option<Vec<Point>> = None;
    let mut buffer = String::new();
    loop {
        let n = io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read stdin");
        if n == 0 {
            // End of input
            break;
        }
        if buffer.trim().len() == 0 {
            continue;
        }
        if buffer.trim().starts_with("---") {
            match current_scanner {
                None => (),
                Some(scanner) => scanners.push(scanner),
            }
            current_scanner = Some(Vec::new());
        } else {
            let reading: Vec<i64> = buffer
                .trim()
                .split(",")
                .map(|x| {
                    x.parse::<i64>()
                        .expect(&format!("Invalid integer: {}", &x[..])[..])
                })
                .collect();
            current_scanner = match current_scanner {
                None => None,
                Some(mut scanner) => {
                    scanner.push(Point::new(reading));
                    Some(scanner)
                }
            };
        }
        buffer.clear();
    }
    match current_scanner {
        None => (),
        Some(scanner) => scanners.push(scanner),
    }
    scanners
}

fn icos(rot_90deg: usize) -> i64 {
    match rot_90deg {
        0 => 1,
        1 => 0,
        2 => -1,
        3 => 0,
        _ => panic!("Invalid"),
    }
}

fn isin(rot_90deg: usize) -> i64 {
    match rot_90deg {
        0 => 0,
        1 => 1,
        2 => 0,
        3 => -1,
        _ => panic!("Invalid"),
    }
}

fn mm(
    x: (i64, i64, i64, i64, i64, i64, i64, i64, i64),
    y: (i64, i64, i64, i64, i64, i64, i64, i64, i64),
) -> (i64, i64, i64, i64, i64, i64, i64, i64, i64) {
    (
        x.0 * y.0 + x.1 * y.3 + x.2 * y.6,
        x.0 * y.1 + x.1 * y.4 + x.2 * y.7,
        x.0 * y.2 + x.1 * y.5 + x.2 * y.8,
        x.3 * y.0 + x.4 * y.3 + x.5 * y.6,
        x.3 * y.1 + x.4 * y.4 + x.5 * y.7,
        x.3 * y.2 + x.4 * y.5 + x.5 * y.8,
        x.6 * y.0 + x.7 * y.3 + x.8 * y.6,
        x.6 * y.1 + x.7 * y.4 + x.8 * y.7,
        x.6 * y.2 + x.7 * y.5 + x.8 * y.8,
    )
}

fn rotation_matrices() -> Vec<(i64, i64, i64, i64, i64, i64, i64, i64, i64)> {
    let mut found: HashSet<(i64, i64, i64, i64, i64, i64, i64, i64, i64)> = HashSet::new();
    for rot_x in 0..4 {
        let x_vec = (
            1,
            0,
            0,
            0,
            icos(rot_x),
            -isin(rot_x),
            0,
            isin(rot_x),
            icos(rot_x),
        );
        for rot_y in 0..4 {
            let y_vec = (
                icos(rot_y),
                0,
                isin(rot_y),
                0,
                1,
                0,
                -isin(rot_y),
                0,
                icos(rot_y),
            );
            let xy_vec = mm(x_vec, y_vec);
            for rot_z in 0..4 {
                let z_vec = (
                    icos(rot_z),
                    -isin(rot_z),
                    0,
                    isin(rot_z),
                    icos(rot_z),
                    0,
                    0,
                    0,
                    1,
                );
                let xyz_vec = mm(xy_vec, z_vec);
                found.insert(xyz_vec);
            }
        }
    }
    Vec::from_iter(found.iter().cloned())
}

fn reduce_positions(
    scanners: &Vec<Vec<Point>>,
) -> (Vec<Vec<Point>>, Option<((usize, usize), Mapping)>) {
    if scanners.len() == 1 {
        return (scanners.clone(), None);
    }
    let rotation_mats = rotation_matrices();
    for scanner_i in 0..(scanners.len() - 1) {
        for scanner_j in (scanner_i + 1)..scanners.len() {
            println!(
                "Looking for pairing between scanners {} and {}",
                scanner_i, scanner_j
            );
            let readings1 = &scanners[scanner_i];
            let readings2 = &scanners[scanner_j];
            let mut trials = 0;

            let old_point_set: HashSet<Point> = HashSet::from_iter(readings1.iter().cloned());

            let mut maximal_overlap: usize = 0;
            for anchor_i in 0..readings1.len() {
                for anchor_j in 0..readings2.len() {
                    for rot_mat in rotation_mats.iter() {
                        let rotated_points: Vec<Point> = readings2
                            .iter()
                            .map(|old| old.transform(*rot_mat))
                            .collect();
                        let anchor_i_point = readings1[anchor_i];
                        let anchor_j_point = rotated_points[anchor_j];
                        let offset = anchor_i_point - anchor_j_point;

                        let new_point_set: HashSet<Point> =
                            HashSet::from_iter(rotated_points.iter().map(|point| *point + offset));
                        let overlap = new_point_set.intersection(&old_point_set).count();
                        if overlap >= 12 {
                            // Combine
                            println!("Reduce");
                            let to_add =
                                Vec::from_iter(new_point_set.difference(&old_point_set).cloned());
                            let new_scanners = scanners
                                .iter()
                                .enumerate()
                                .filter(|x| x.0 != scanner_j)
                                .map(|x| {
                                    if x.0 == scanner_i {
                                        let mut new_vec = x.1.clone();
                                        new_vec.extend_from_slice(&to_add.as_slice());
                                        new_vec
                                    } else {
                                        Vec::from_iter(x.1.iter().cloned())
                                    }
                                })
                                .collect();
                            return (
                                new_scanners,
                                Some(((scanner_j, scanner_i), Mapping::new(*rot_mat, offset))),
                            );
                        }
                        maximal_overlap = cmp::max(overlap, maximal_overlap);
                        trials += 1;
                    }
                }
            }
            println!("Trials: {}, max overlap: {}", trials, maximal_overlap);
        }
    }
    panic!("Couldn't reduce");
}

fn main() {
    let original_scanners = parse();
    let mut scanners = original_scanners.clone();
    let mut current_index_to_original_indices: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut original_index_to_current_index: HashMap<usize, Mapping> = HashMap::new();
    for i in 0..scanners.len() {
        let mut set = HashSet::new();
        set.insert(i);
        current_index_to_original_indices.insert(i, set);
        original_index_to_current_index.insert(i, Mapping::identity());
    }
    while scanners.len() > 1 {
        let out = reduce_positions(&scanners);
        scanners = out.0;
        match out.1 {
            None => (),
            Some(((removed, merged), mapping)) => {
                let new_indices = current_index_to_original_indices[&removed].clone();

                let new_mapping: HashMap<usize, HashSet<usize>> = current_index_to_original_indices
                    .iter()
                    .filter(|x| *x.0 != removed)
                    .map(|(old_index, old_values)| {
                        let new_index = if *old_index > removed {
                            *old_index - 1
                        } else {
                            *old_index
                        };
                        let mut new_values = old_values.clone();
                        if *old_index == merged {
                            new_values.extend(new_indices.iter().cloned());
                        }
                        (new_index, new_values)
                    })
                    .collect();
                current_index_to_original_indices = new_mapping;

                for index in new_indices {
                    let new_mapping = original_index_to_current_index[&index].compose(&mapping);
                    original_index_to_current_index.insert(index, new_mapping);
                }
            }
        }
    }

    let scanner_locations: Vec<Point> = (0..original_scanners.len())
        .map(|i| {
            let mapping = &original_index_to_current_index[&i];
            let scanner_location = mapping.offset_vector;
            scanner_location
        })
        .collect();

    let mut distances: Vec<i64> = Vec::new();
    for i in 0..(scanner_locations.len() - 1) {
        for j in (i + 1)..scanner_locations.len() {
            distances.push(scanner_locations[i].manhattan(&scanner_locations[j]));
        }
    }

    println!("Beacons: {}", scanners[0].len());
    println!("Max distance: {}", distances.iter().max().unwrap());
}
