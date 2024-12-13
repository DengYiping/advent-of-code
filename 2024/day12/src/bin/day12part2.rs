use std::collections::HashSet;

use advent_of_code_2024_day_12::{
    coordinate::{Board, Coordinate, MoveDirection, Wall},
    reading::read,
};

struct AreaAndPerimeter {
    area: u64,
    perimeter: u64,
}

impl AreaAndPerimeter {
    fn price(&self) -> u64 {
        self.area * self.perimeter
    }
}

fn main() {
    let board = read();
    let board = Board::wrap(board);

    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut result: u64 = 0;
    for i in 0..board.m {
        for j in 0..board.n {
            let coordinate = Coordinate::new(i, j, board.clone());
            result += calculate(coordinate, &mut visited).price()
        }
    }
    println!("Result: {}", result);
}

fn calculate(coordinate: Coordinate, visited: &mut HashSet<Coordinate>) -> AreaAndPerimeter {
    let mut area_perimeter = AreaAndPerimeter {
        area: 0,
        perimeter: 0,
    };
    dfs(coordinate, visited, &mut HashSet::new(), &mut area_perimeter);
    area_perimeter
}

fn dfs(
    coordinate: Coordinate,
    visited: &mut HashSet<Coordinate>,
    walls: &mut HashSet<Wall>,
    area_perimeter: &mut AreaAndPerimeter,
) {
    if !visited.contains(&coordinate) {
        visited.insert(coordinate.clone());
        area_perimeter.area += 1;
        area_perimeter.perimeter += coordinate.sides(walls);
        for direction in [
            MoveDirection::Up,
            MoveDirection::Down,
            MoveDirection::Left,
            MoveDirection::Right,
        ] {
            if let Some(new_coordinate) = coordinate.move_to(&direction) {
                if new_coordinate.get_value() == coordinate.get_value() {
                    dfs(new_coordinate, visited, walls, area_perimeter);
                }
            }
        }
    }
}
