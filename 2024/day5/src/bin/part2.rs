use std::cmp::Ordering;

use advent_of_code_2024_day5::reading;

fn main() {
    let input = reading::read();
    let mut result = 0;
    for case in input.cases {
        let mut sorted = case.clone();
        sorted.sort_by(|a, b| {
            if let Some(should_after) = input.should_not_shown_before.get(a) {
                if should_after.contains(b) {
                    return Ordering::Less;
                }
            }
            if let Some(should_after) = input.should_not_shown_before.get(b) {
                if should_after.contains(a) {
                    return Ordering::Greater;
                }
            }
            return Ordering::Equal;
        });
        if sorted.iter().zip(case.iter()).filter(|(a, b)| a == b).count() != case.len() {
            result += if case.len() % 2 == 0 {
                (sorted[case.len() / 2] + sorted[case.len() / 2 - 1]) / 2
            } else {
                sorted[case.len() / 2]
            }
        }
    }

    println!("Result: {}", result)
}

