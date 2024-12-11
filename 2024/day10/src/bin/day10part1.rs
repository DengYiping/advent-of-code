use std::collections::{HashMap, HashSet};

use advent_of_code_2024_day_10::{coordinate::Coordinate, reading::read, solution::{find_trailhead, score}};

fn main() {
    let input = read();
    let mut result: usize = 0;
    let mut mem: HashMap<Coordinate, HashSet<Coordinate>> = HashMap::new();
    for coordinate in find_trailhead(&input) {
        let current_score = score(&input, &coordinate, &mut mem, &mut HashSet::new());
        result += current_score;
    }
    println!("Result: {}", result);
}
