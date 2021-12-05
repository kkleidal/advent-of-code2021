use std::collections::HashMap;
use std::io;

fn parse_segments() -> Vec<((i32, i32), (i32, i32))> {
    let mut buffer = String::new();
    let mut segments: Vec<((i32, i32), (i32, i32))> = Vec::new();
    loop {
        let n = io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read stdin");
        if n == 0 {
            // End of input
            break;
        }
        // 0,9 -> 5,9
        let segment_vec: Vec<(i32, i32)> = buffer
            .trim()
            .split(" -> ")
            .map(|pair| {
                let vec: Vec<i32> = pair
                    .trim()
                    .split(",")
                    .map(|x| {
                        let val: i32 = x.parse().expect("Invalid integer");
                        val
                    })
                    .collect();
                if vec.len() == 2 {
                    return (vec[0], vec[1]);
                } else {
                    panic!("Invalid pair: {}", pair);
                }
            })
            .collect();

        if segment_vec.len() == 2 {
            let segment = (segment_vec[0], segment_vec[1]);
            segments.push(segment);
        } else {
            panic!("Invalid segment: {:?}", segment_vec);
        }
        buffer.clear();
    }
    segments
}

fn cmp(x: i32, y: i32) -> i32 {
    if y > x {
        return 1;
    } else if y == x {
        return 0;
    } else {
        return -1;
    }
}

fn segments_to_coord_counts(
    segments: &[((i32, i32), (i32, i32))],
    only_unidirectional: bool,
) -> HashMap<(i32, i32), usize> {
    let mut counts: HashMap<(i32, i32), usize> = HashMap::new();
    for &(start, end) in segments {
        let delta = (cmp(start.0, end.0), cmp(start.1, end.1));
        if only_unidirectional && !(delta.0 == 0 || delta.1 == 0) {
            continue;
        }
        let mut current = start;
        loop {
            let reference = counts.entry(current).or_insert(0);
            *reference += 1;

            if current == end {
                break;
            }
            current = (current.0 + delta.0, current.1 + delta.1);
        }
    }
    return counts;
}

fn count_duplicates(counts: &HashMap<(i32, i32), usize>) -> usize {
    let mut avoid: usize = 0;
    for (pos, count) in counts {
        if *count >= 2 {
            avoid += 1;
        }
    }
    avoid
}

fn main() {
    let segments = parse_segments();
    let coord_counts = segments_to_coord_counts(segments.as_slice(), true);
    println!("Part 1: {:?}", count_duplicates(&coord_counts));

    let coord_counts_diag = segments_to_coord_counts(segments.as_slice(), false);
    println!("Part 2: {:?}", count_duplicates(&coord_counts_diag));
}
