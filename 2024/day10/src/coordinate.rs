use std::{
    hash::{Hash, Hasher},
    rc::Rc,
};

#[derive(Debug, Clone)]
pub struct Coordinate {
    i: usize,
    j: usize,
    board_size: Rc<BoardSize>,
}

#[derive(Debug)]
pub struct BoardSize {
    m: usize,
    n: usize,
}

impl BoardSize {
    pub fn new(m: usize, n: usize) -> Self {
        BoardSize { m, n }
    }
}

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

impl Coordinate {
    pub fn new(i: usize, j: usize, board_size: Rc<BoardSize>) -> Self {
        assert!(i < board_size.m && j < board_size.n, "Invalid coordinate!");
        Coordinate { i, j, board_size }
    }

    pub fn get<T: Copy>(&self, board: &Vec<Vec<T>>) -> T {
        board[self.i][self.j]
    }

    pub fn move_direction(&self, direction: MoveDirection) -> Option<Coordinate> {
        match direction {
            MoveDirection::Up => {
                if self.i == 0 {
                    None
                } else {
                    Some(Coordinate {
                        i: self.i - 1,
                        j: self.j,
                        board_size: self.board_size.clone(),
                    })
                }
            }
            MoveDirection::Down => {
                if self.i == self.board_size.m - 1 {
                    None
                } else {
                    Some(Coordinate {
                        i: self.i + 1,
                        j: self.j,
                        board_size: self.board_size.clone(),
                    })
                }
            }
            MoveDirection::Left => {
                if self.j == 0 {
                    None
                } else {
                    Some(Coordinate {
                        i: self.i,
                        j: self.j - 1,
                        board_size: self.board_size.clone(),
                    })
                }
            }
            MoveDirection::Right => {
                if self.j == self.board_size.n - 1 {
                    None
                } else {
                    Some(Coordinate {
                        i: self.i,
                        j: self.j + 1,
                        board_size: self.board_size.clone(),
                    })
                }
            }
        }
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

pub fn get_all_direction() -> Vec<MoveDirection> {
    vec![
        MoveDirection::Up,
        MoveDirection::Down,
        MoveDirection::Left,
        MoveDirection::Right,
    ]
}
