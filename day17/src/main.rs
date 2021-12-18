use std::cmp;

fn find_highest_touched(x_min: i64, x_max: i64, y_min: i64, y_max: i64) -> (i64, usize) {
    let x_start = ((-1.0 + (1.0 + 8.0 * (x_min as f64)).sqrt()) / 2.0).ceil() as i64;
    // println!("Start at x={}", x_start);
    let mut max_hit_y: i64 = 0;
    let mut distinct: usize = 0;
    for dx_init in x_start..(x_max + 1) {
        let mut dy_init: i64 = y_min;
        let mut previously_on_target: bool = false;
        let mut stop_at = -1;
        while stop_at < 0 || dy_init < stop_at {
            let mut dx = dx_init;
            let mut dy = dy_init;
            let mut y: i64 = 0;
            let mut x: i64 = 0;
            let mut hit_target: bool = false;
            let mut overshoot: bool = true;
            let mut max_y: i64 = y;
            while y >= y_min && x <= x_max {
                if y >= y_min && y <= y_max && x >= x_min && x <= x_max {
                    hit_target = true;
                    break;
                }
                if (x < x_min && y < y_min && dy < 0) || (x >= x_min && x <= x_max && y <= y_max) {
                    overshoot = false;
                }
                y += dy;
                x += dx;
                max_y = cmp::max(max_y, y);
                dy -= 1;
                if dx > 0 {
                    dx -= 1; // Assumes only positive x
                }
            }
            if (x < x_min && y < y_min && dy < 0) || (x >= x_min && x <= x_max && y <= y_max) {
                overshoot = false;
            }
            if hit_target {
                // Hit
                previously_on_target = true;
                distinct += 1;
                max_hit_y = cmp::max(max_y, max_hit_y);
            } else if stop_at < 0 {
                if overshoot {
                    // Note(ken.leidal): I'm not proud of this. It's super janky. But I couldn't
                    // find a reliable termination condition and this found the answer...
                    stop_at = dy_init + 1000;
                } else if previously_on_target {
                    // Note(ken.leidal): I'm not proud of this. It's super janky. But I couldn't
                    // find a reliable termination condition and this found the answer...
                    stop_at = dy_init + 1000;
                }
            }
            dy_init += 1;
        }
    }
    (max_hit_y, distinct)
}

fn main() {
    println!(
        "Part 1, example: {}",
        find_highest_touched(20, 30, -10, -5).0
    );
    println!(
        "Part 1, mine: {}",
        find_highest_touched(175, 227, -134, -79).0
    );
    println!(
        "Part 2, example: {}",
        find_highest_touched(20, 30, -10, -5).1
    );
    println!(
        "Part 2, mine: {}",
        find_highest_touched(175, 227, -134, -79).1
    );
}
