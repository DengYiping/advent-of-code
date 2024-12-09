use day4::reading;

static WORD: &str = "XMAS";

fn main() {
    let input = reading::read();
    assert!(input.len() > 0, "Failed to read1");
    let m = input.len();
    let n = input[0].len();
    let mut result = 0;

    for i in 0..m {
        for j in 0..n {
            let bound = Bound { m, n };
            let coordinate = Coordinate {
                i: i.try_into().unwrap(),
                j: j.try_into().unwrap(),
            };
            result += search(&input, coordinate, &bound);
        }
    }

    println!("Result: {}", result);
}

struct Bound {
    m: usize,
    n: usize,
}

#[derive(Clone, Copy, Debug)]
struct Coordinate {
    i: i32,
    j: i32,
}

impl Coordinate {
    fn validate(self, bound: &Bound) -> Option<Coordinate> {
        if self.i >= 0
            && self.i < bound.m.try_into().unwrap()
            && self.j >= 0
            && self.j < bound.n.try_into().unwrap()
        {
            Some(self)
        } else {
            None
        }
    }

    fn get(&self, board: &Vec<String>) -> u8 {
        board[self.i as usize].as_bytes()[self.j as usize]
    }

    fn is_right_char(&self, board: &Vec<String>, word: &str, depth: usize) -> bool {
        self.get(board) == word.as_bytes()[depth]
    }
}

enum SearchDirection {
    Left,
    Right,
    Top,
    Bottom,
    TopLeftDiagonal,
    TopRightDiagonal,
    BottomLeftDiagonal,
    BottomRightDiagonal,
}

impl SearchDirection {
    fn next(&self, coordinate: &Coordinate, bound: &Bound) -> Option<Coordinate> {
        match self {
            SearchDirection::Left => Coordinate {
                i: coordinate.i,
                j: coordinate.j - 1,
            }
            .validate(bound),
            SearchDirection::Right => Coordinate {
                i: coordinate.i,
                j: coordinate.j + 1,
            }
            .validate(bound),
            SearchDirection::Top => Coordinate {
                i: coordinate.i - 1,
                j: coordinate.j,
            }
            .validate(bound),
            SearchDirection::Bottom => Coordinate {
                i: coordinate.i + 1,
                j: coordinate.j,
            }
            .validate(bound),
            SearchDirection::TopLeftDiagonal => Coordinate {
                i: coordinate.i - 1,
                j: coordinate.j - 1,
            }
            .validate(bound),
            SearchDirection::TopRightDiagonal => Coordinate {
                i: coordinate.i - 1,
                j: coordinate.j + 1,
            }
            .validate(bound),
            SearchDirection::BottomLeftDiagonal => Coordinate {
                i: coordinate.i + 1,
                j: coordinate.j - 1,
            }
            .validate(bound),
            SearchDirection::BottomRightDiagonal => Coordinate {
                i: coordinate.i + 1,
                j: coordinate.j + 1,
            }
            .validate(bound),
        }
    }
}

fn search(board: &Vec<String>, coordinate: Coordinate, bound: &Bound) -> u32 {
    let directions = vec![
        SearchDirection::Left,
        SearchDirection::Right,
        SearchDirection::Top,
        SearchDirection::Bottom,
        SearchDirection::TopLeftDiagonal,
        SearchDirection::TopRightDiagonal,
        SearchDirection::BottomLeftDiagonal,
        SearchDirection::BottomRightDiagonal,
    ];
    let mut result = 0;

    for direction in directions {
        if search_in_direction(&board, &direction, &bound, coordinate.clone(), WORD, 0) {
            result += 1;
        }
    }

    result
}

fn search_in_direction(
    board: &Vec<String>,
    direction: &SearchDirection,
    bound: &Bound,
    coordinate: Coordinate,
    word: &str,
    depth: usize,
) -> bool {
    if depth >= word.len() {
        return true;
    }
    if coordinate.is_right_char(board, word, depth) {
        match direction.next(&coordinate, bound) {
            Some(new) => search_in_direction(board, direction, bound, new, word, depth + 1),
            None => if depth == word.len() - 1 {
                true
            } else {
                false
            },
        }
    } else {
        false
    }
}
