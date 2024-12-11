use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::coordinate::{get_all_direction, BoardSize, Coordinate};

static ZERO: u8 = '0' as u8;

pub fn create_board_size(board: &Vec<Vec<u8>>) -> Rc<BoardSize> {
    Rc::new(BoardSize::new(board.len(), board[0].len()))
}

pub fn find_trailhead(board: &Vec<Vec<u8>>) -> Vec<Coordinate> {
    let board_size = create_board_size(board);
    let mut result = vec![];
    for (i, row) in board.iter().enumerate() {
        for (j, &data) in row.iter().enumerate() {
            if data == ZERO {
                result.push(Coordinate::new(i, j, board_size.clone()));
            }
        }
    }
    result
}

pub fn score(
    board: &Vec<Vec<u8>>,
    coordinate: &Coordinate,
    mem: &mut HashMap<Coordinate, HashSet<Coordinate>>,
    visited: &mut HashSet<Coordinate>,
) -> usize {
    if visited.contains(coordinate) {
        return 0;
    }
    visited.insert(coordinate.clone());
    if !mem.contains_key(coordinate) {
        let current = coordinate.get(board);
        if current == ('9' as u8) {
            let mut score_set = HashSet::new();
            score_set.insert(coordinate.clone());
            mem.insert(coordinate.clone(), score_set);
        } else {
            let mut score_set: HashSet<Coordinate> = HashSet::new();
            for direction in get_all_direction() {
                if let Some(new_coordinate) = coordinate.move_direction(direction) {
                    if !visited.contains(&new_coordinate)
                        && new_coordinate.get(board) == (current + 1)
                    {
                        score(board, &new_coordinate, mem, visited);
                        score_set.extend(mem.get(&new_coordinate).unwrap().iter().cloned());
                    }
                }
            }
            mem.insert(coordinate.clone(), score_set);
        }
    }
    visited.remove(coordinate);
    mem.get(coordinate).unwrap().len()
}

pub fn score_part_2(
    board: &Vec<Vec<u8>>,
    coordinate: &Coordinate,
    visited: &mut HashSet<Coordinate>,
) -> usize {
    if visited.contains(coordinate) {
        return 0;
    }
    if coordinate.get(board) == ('9' as u8) {
        1
    } else {
        let mut acc: usize = 0;
        visited.insert(coordinate.clone());
        for direction in get_all_direction() {
            if let Some(new_coordinate) = coordinate.move_direction(direction) {
                if !visited.contains(&new_coordinate)
                    && new_coordinate.get(board) == (coordinate.get(board) + 1)
                {
                    acc += score_part_2(board, &new_coordinate, visited)
                }
            }
        }
        visited.remove(coordinate);
        acc
    }
}
