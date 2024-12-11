use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    i: usize,
    j: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec2D {
    i: i64,
    j: i64,
}

impl Vec2D {
    fn from_coordinate(coordinate: &Coordinate) -> Vec2D {
        Vec2D {
            i: coordinate.i as i64,
            j: coordinate.j as i64,
        }
    }

    fn to_coordinate(&self) -> Coordinate {
        Coordinate::new(self.i as usize, self.j as usize)
    }

    fn is_in_board(&self, board_size: &BoardSize) -> bool {
        self.i >= 0 && self.j >= 0 && self.i < board_size.m as i64 && self.j < board_size.n as i64
    }

    fn normalize_by_gcd(self) -> Vec2D {
        let gcd = gcd(self.i, self.j);
        Vec2D {
            i: self.i / gcd,
            j: self.j / gcd,
        }
    }
}

impl Add<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn add(self, other: Vec2D) -> Vec2D {
        Vec2D {
            i: self.i + other.i,
            j: self.j + other.j,
        }
    }
}

impl Sub<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn sub(self, other: Vec2D) -> Vec2D {
        Vec2D {
            i: self.i - other.i,
            j: self.j - other.j,
        }
    }
}

#[derive(Debug)]
pub struct BoardSize {
    m: usize,
    n: usize,
}

impl BoardSize {
    pub fn new(m: usize, n: usize) -> BoardSize {
        BoardSize { m, n }
    }
    fn is_in_board(&self, coordinate: &Coordinate) -> bool {
        coordinate.i < self.m && coordinate.j < self.n
    }
}

impl Coordinate {
    pub fn new(i: usize, j: usize) -> Coordinate {
        Coordinate { i, j }
    }

    fn reflection(&self, pivot: &Coordinate, board_size: &BoardSize) -> Option<Coordinate> {
        let i: i64 = (pivot.i as i64) - ((self.i as i64) - (pivot.i as i64));
        let j: i64 = (pivot.j as i64) - ((self.j as i64) - (pivot.j as i64));
        if i >= 0 && j >= 0 {
            let coordinate = Coordinate {
                i: i as usize,
                j: j as usize,
            };
            if board_size.is_in_board(&coordinate) {
                Some(coordinate)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn reflection_in_line(&self, other: &Coordinate, board_size: &BoardSize) -> Vec<Coordinate> {
        let left = Vec2D::from_coordinate(self);
        let right = Vec2D::from_coordinate(other);

        let mut reflections = Vec::new();

        let delta = (right - left).normalize_by_gcd();
        let mut current_1 = left;
        let mut current_2 = current_1 - delta;
        while current_1.is_in_board(board_size) {
            let reflection = current_1.to_coordinate();
            reflections.push(reflection);
            current_1 = current_1 + delta;
        }

        while current_2.is_in_board(board_size) {
            let reflection = current_2.to_coordinate();
            reflections.push(reflection);
            current_2 = current_2 - delta;
        }

        reflections
    }

    pub fn reflections(&self, other: &Coordinate, board_size: &BoardSize) -> Vec<Coordinate> {
        let mut reflections = Vec::new();
        if let Some(reflection) = self.reflection(other, board_size) {
            reflections.push(reflection);
        }
        if let Some(reflection) = other.reflection(self, board_size) {
            reflections.push(reflection);
        }
        reflections
    }
}

pub fn all_reflections(
    coordinates: &Vec<Coordinate>,
    board_size: &BoardSize,
    reflections: &mut HashSet<Coordinate>,
    reflection_generator: fn(&Coordinate, &Coordinate, &BoardSize) -> Vec<Coordinate>,
) {
    for i in 0..coordinates.len() {
        for j in (i + 1)..coordinates.len() {
            for reflection in reflection_generator(&coordinates[i], &coordinates[j], board_size) {
                reflections.insert(reflection);
            }
        }
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if a < 0 {
        gcd(-1 * a, b)
    } else if b < 0 {
        gcd(a, -1 * b)
    } else if a > b {
        gcd(b, a)
    } else if a == 0 {
        b
    } else {
        gcd(a, b % a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflection() {
        let board_size = BoardSize::new(3, 3);
        let coordinate = Coordinate::new(0, 0);
        let pivot = Coordinate::new(1, 1);
        let reflection = coordinate.reflection(&pivot, &board_size).unwrap();
        assert_eq!(reflection, Coordinate::new(2, 2));

        let pivot = Coordinate::new(0, 1);
        let reflection = coordinate.reflection(&pivot, &board_size).unwrap();
        assert_eq!(reflection, Coordinate::new(0, 2));
    }

    #[test]
    fn test_reverse_reflection() {
        let board_size = BoardSize::new(3, 3);
        let coordinate = Coordinate::new(2, 2);
        let pivot = Coordinate::new(1, 1);
        let reflection = coordinate.reflection(&pivot, &board_size).unwrap();
        assert_eq!(reflection, Coordinate::new(0, 0))
    }

    #[test]
    fn test_out_of_bound() {
        let board_size = BoardSize::new(2, 2);
        let coordinate = Coordinate::new(0, 0);
        let pivot = Coordinate::new(1, 1);
        let reflection = coordinate.reflection(&pivot, &board_size);
        assert_eq!(reflection, None)
    }

    #[test]
    fn test_reflections() {
        let board_size = BoardSize::new(5, 5);
        let coordinate1 = Coordinate::new(1, 1);
        let coordinate2 = Coordinate::new(2, 2);
        let reflections = coordinate1.reflections(&coordinate2, &board_size);
        assert_eq!(reflections.len(), 2);
        assert!(reflections.contains(&Coordinate::new(3, 3)));
        assert!(reflections.contains(&Coordinate::new(0, 0)));
    }

    #[test]
    fn test_all_reflections() {
        let board_size = BoardSize::new(5, 5);
        let coordinates = vec![Coordinate::new(1, 1), Coordinate::new(2, 2)];
        let mut reflections: HashSet<Coordinate> = HashSet::new();
        all_reflections(&coordinates, &board_size, &mut reflections, |x, y, board| x.reflections(y, board));
        assert_eq!(reflections.len(), 2);
        assert!(reflections.contains(&Coordinate::new(3, 3)));
        assert!(reflections.contains(&Coordinate::new(0, 0)));
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(2, 3), 1);
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(3, 6), 3);
        assert_eq!(gcd(6, 4), 2);
        assert_eq!(gcd(-2, -4), 2);
        assert_eq!(gcd(-2, 4), 2);
    }
}
