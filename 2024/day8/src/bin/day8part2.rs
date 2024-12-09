use std::collections::{HashMap, HashSet};

use advent_of_code_2024_day_8::{antinodes::{all_reflections, BoardSize, Coordinate}, reading::read};

fn main() {
    let input = read();
    let board_size = BoardSize::new(input.len(), input[0].len());
    let mut freq_to_coordinates: HashMap<u8, Vec<Coordinate>> = HashMap::new();
    for (i, row) in input.iter().enumerate() {
        for (j, data) in row.iter().enumerate() {
            if let Some(freq) = data {
                freq_to_coordinates.entry(*freq)
                    .and_modify(|e| {e.push(Coordinate::new(i, j))})
                    .or_insert_with(|| vec![Coordinate::new(i, j)]);
            }
        }
    }

    let mut reflections: HashSet<Coordinate> = HashSet::new();
    for (_, coordinates) in freq_to_coordinates {
        all_reflections(&coordinates, &board_size, &mut reflections, |l, r, b| l.reflection_in_line(r, b));
    }
    println!("Result: {}", reflections.len());
}
