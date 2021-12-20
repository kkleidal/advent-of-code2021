use std::collections::HashSet;
use std::io;

#[derive(Debug, Clone)]
struct Image {
    different_from_default: HashSet<(isize, isize)>,
    default_pixel: bool,
}

impl Image {
    fn from_array(arr: &[Vec<bool>]) -> Self {
        let mut different: HashSet<(isize, isize)> = HashSet::new();
        for i in 0..arr.len() {
            for j in 0..arr[i].len() {
                if arr[i][j] {
                    different.insert((i.try_into().unwrap(), j.try_into().unwrap()));
                }
            }
        }
        Self {
            different_from_default: different,
            default_pixel: false,
        }
    }

    fn attended_pixels_for_update(&self) -> HashSet<(isize, isize)> {
        let mut attended: HashSet<(isize, isize)> = HashSet::new();
        for (i, j) in self.different_from_default.iter() {
            for di in -1..2 {
                for dj in -1..2 {
                    attended.insert((i + di, j + dj));
                }
            }
        }
        attended
    }

    fn enhance(&self, algorithm: &[bool]) -> Image {
        let new_default = if self.default_pixel {
            algorithm[511]
        } else {
            algorithm[0]
        };
        let mut different: HashSet<(isize, isize)> = HashSet::new();
        for (i, j) in self.attended_pixels_for_update().iter().cloned() {
            let mut index: usize = 0;
            for di in -1..2 {
                for dj in -1..2 {
                    let key = (i + di, j + dj);
                    let value = if self.different_from_default.contains(&key) {
                        !self.default_pixel
                    } else {
                        self.default_pixel
                    };
                    index = (index << 1) | (if value { 1 } else { 0 });
                }
            }
            let new_value = algorithm[index];
            if new_value != new_default {
                different.insert((i, j));
            }
        }
        Image {
            different_from_default: different,
            default_pixel: new_default,
        }
    }

    // fn render(&self) {
    //     let min_i = self.different_from_default.iter().cloned().map(|x| x.0).min().unwrap() - 1;
    //     let max_i = self.different_from_default.iter().cloned().map(|x| x.0).max().unwrap() + 2;
    //     let min_j = self.different_from_default.iter().cloned().map(|x| x.1).min().unwrap() - 1;
    //     let max_j = self.different_from_default.iter().cloned().map(|x| x.1).max().unwrap() + 2;
    //     for i in min_i..max_i {
    //         for j in min_j..max_j {
    //             let key = (i, j);
    //             let value = if self.different_from_default.contains(&key) {
    //                 !self.default_pixel
    //             } else {
    //                 self.default_pixel
    //             };
    //             print!("{}", if value { "#" } else { "." });
    //         }
    //         println!();
    //     }
    //     println!();
    // }

    fn number_lit(&self) -> Option<usize> {
        if self.default_pixel {
            None
        } else {
            Some(self.different_from_default.len())
        }
    }
}

fn parse() -> (Vec<bool>, Image) {
    let mut alg: Vec<bool> = Vec::new();
    let mut img_arr: Vec<Vec<bool>> = Vec::new();
    let mut state = 0;
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
            state += 1;
            continue;
        }
        let binary: Vec<bool> = buffer
            .trim()
            .chars()
            .filter(|c| match c {
                '#' => true,
                '.' => true,
                _ => false,
            })
            .map(|c| match c {
                '#' => true,
                '.' => false,
                _ => false,
            })
            .collect();
        match state {
            0 => alg.extend_from_slice(&binary[..]),
            1 => img_arr.push(binary),
            _ => panic!("Invalid state"),
        }
        buffer.clear();
    }
    (alg, Image::from_array(&img_arr.as_slice()))
}

fn main() {
    println!("Hello, world!");
    let (algorithm, original_image) = parse();

    let mut image = original_image.clone();
    // image.render();
    for _ in 0..2 {
        image = image.enhance(&algorithm.as_slice());
        // image.render();
    }
    println!(
        "Number lit after 2: {}",
        image.number_lit().expect("Uh oh, the number was infinite")
    );

    image = original_image.clone();
    for _ in 0..50 {
        image = image.enhance(&algorithm.as_slice());
    }
    println!(
        "Number lit after 50: {}",
        image.number_lit().expect("Uh oh, the number was infinite")
    );
}
