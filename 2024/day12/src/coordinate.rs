use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    rc::Rc,
};

pub struct Board {
    board: Vec<Vec<u8>>,
    pub m: usize,
    pub n: usize,
}

#[derive(Clone)]
pub struct Coordinate {
    pub i: usize,
    pub j: usize,
    board: Rc<Board>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Wall {
    top_left_coordinate: Coordinate,
    direction: MoveDirection,
}

impl Board {
    pub fn wrap(board: Vec<Vec<u8>>) -> Rc<Board> {
        let m = board.len();
        let n = board[0].len();
        Rc::new(Board { board, m, n })
    }
}

impl Coordinate {
    pub fn new(i: usize, j: usize, board: Rc<Board>) -> Coordinate {
        Coordinate { i, j, board }
    }

    pub fn move_to(&self, direction: &MoveDirection) -> Option<Coordinate> {
        let i = self.i;
        let j = self.j;
        let m = self.board.m;
        let n = self.board.n;

        match direction {
            MoveDirection::Up => {
                if i == 0 {
                    None
                } else {
                    Some(Coordinate::new(i - 1, j, self.board.clone()))
                }
            }
            MoveDirection::Down => {
                if i == m - 1 {
                    None
                } else {
                    Some(Coordinate::new(i + 1, j, self.board.clone()))
                }
            }
            MoveDirection::Left => {
                if j == 0 {
                    None
                } else {
                    Some(Coordinate::new(i, j - 1, self.board.clone()))
                }
            }
            MoveDirection::Right => {
                if j == n - 1 {
                    None
                } else {
                    Some(Coordinate::new(i, j + 1, self.board.clone()))
                }
            }
        }
    }

    pub fn get_value(&self) -> u8 {
        self.board.board[self.i][self.j]
    }

    pub fn perimeter(&self) -> u64 {
        let mut perimeter = 0;
        for direction in [
            MoveDirection::Up,
            MoveDirection::Down,
            MoveDirection::Left,
            MoveDirection::Right,
        ] {
            if self.is_wall(&direction) {
                perimeter += 1;
            }
        }
        perimeter
    }

    pub fn is_wall(&self, wall_direction: &MoveDirection) -> bool {
        if let Some(next) = self.move_to(wall_direction) {
            if self.get_value() != next.get_value() {
                true
            } else {
                false
            }
        } else {
            return true;
        }
    }

    pub fn find_wall(&self, wall_direction: &MoveDirection) -> Wall {
        // Keep searching in the top left direction until the wall is ended
        match wall_direction {
            MoveDirection::Left | MoveDirection::Right => {
                if let Some(new_coord) = self.move_to(&MoveDirection::Up) {
                    if new_coord.get_value() == self.get_value()
                        && new_coord.is_wall(wall_direction)
                    {
                        return new_coord.find_wall(wall_direction);
                    }
                }
            }
            MoveDirection::Up | MoveDirection::Down => {
                if let Some(new_coord) = self.move_to(&MoveDirection::Left) {
                    if new_coord.get_value() == self.get_value()
                        && new_coord.is_wall(wall_direction)
                    {
                        return new_coord.find_wall(wall_direction);
                    }
                }
            }
        }
        return Wall {
            top_left_coordinate: self.clone(),
            direction: wall_direction.clone(),
        };
    }

    pub fn sides(&self, walls: &mut HashSet<Wall>) -> u64 {
        let mut sides = 0;
        for direction in [
            MoveDirection::Up,
            MoveDirection::Down,
            MoveDirection::Left,
            MoveDirection::Right,
        ] {
            if self.is_wall(&direction) {
                let wall = self.find_wall(&direction);
                if !walls.contains(&wall) {
                    walls.insert(wall);
                    sides += 1;
                }
            }
        }
        sides
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i && self.j == other.j
    }
}

impl Eq for Coordinate {}

impl Hash for Coordinate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.i.hash(state);
        self.j.hash(state);
    }
}
