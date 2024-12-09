use std::collections::{HashMap, HashSet};

use advent_of_code_2024_day5::reading;

fn main() {
    let input = reading::read();
    let mut result = 0;
    for case in input.cases {
        if is_valid(&case, &input.should_not_shown_before) {
            result += if case.len() % 2 == 0 {
                (case[case.len() / 2] + case[case.len() / 2 - 1]) / 2
            } else {
                case[case.len() / 2]
            }
        }
    }

    println!("Result: {}", result)
}

fn is_valid(case: &Vec<u64>, rules: &HashMap<u64, HashSet<u64>>) -> bool {
    let mut seen: HashSet<u64> = HashSet::new();
    for num in case {
        if let Some(should_not_seen) = rules.get(num) {
            if seen.intersection(should_not_seen).count() > 0 {
                return false;
            }
        }
        seen.insert(num.clone());
    }
    true
}
