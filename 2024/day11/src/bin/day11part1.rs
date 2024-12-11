use std::collections::HashMap;

use advent_of_code_2024_day_11::read;

fn main() {
    let input = read();
    let depth = 75;
    let mut cache = HashMap::new();
    let mut result = 0;
    for num in input {
        result += count_stone(num, depth, &mut cache);
    }
    println!("Result: {}", result);
}

fn count_stone(num: u64, depth: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if depth == 0 {
        return 1;
    }

    if let Some(&count) = cache.get(&(num, depth)) {
        return count;
    }

    let mut count = 0;
    if num == 0 {
        count = count_stone(1, depth - 1, cache);
    } else if format!("{}", num).len() % 2 == 0 {
        let digits = format!("{}", num);
        let left: u64 = digits[0..digits.len()/2].parse().unwrap();
        let right: u64 = digits[digits.len()/2 ..].parse().unwrap();
        count = count_stone(left, depth - 1, cache) + count_stone(right, depth - 1, cache);
    } else {
        count = count_stone(num * 2024, depth - 1, cache);
    }

    cache.insert((num, depth), count);
    count
}
