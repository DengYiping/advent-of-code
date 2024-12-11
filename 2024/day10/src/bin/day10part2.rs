use std::collections::HashSet;

use advent_of_code_2024_day_10::{reading::read, solution::{find_trailhead, score_part_2}};

fn main() {
    let input = read();
    let mut result: usize = 0;
    for coordinate in find_trailhead(&input) {
        let current_score = score_part_2(&input, &coordinate, &mut HashSet::new());
        result += current_score;
    }
    println!("Result: {}", result);
}
