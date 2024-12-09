use std::collections::HashSet;

use advent_of_code_day6::reading;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum HeadDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Coordinate {
    i: usize,
    j: usize,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct State {
    i: usize,
    j: usize,
    direction: HeadDirection,
}

impl State {
    fn visit_until_looped(
        &mut self,
        board: &Vec<Vec<u8>>,
        visited_state: &mut HashSet<State>,
    ) -> bool {
        if visited_state.contains(&self) {
            return true;
        }
        visited_state.insert(self.clone());
        if self.visit(&board) {
            self.visit_until_looped(&board, visited_state)
        } else {
            false
        }
    }

    fn visit_tracked(&mut self, board: &Vec<Vec<u8>>, visited: &mut HashSet<Coordinate>) -> bool {
        visited.insert(self.to_coordinate());
        self.visit(&board)
    }

    fn visit(&mut self, board: &Vec<Vec<u8>>) -> bool {
        while self.can_move_within_bound(board) && self.will_hit_block(board) {
            self.rotate();
        }
        if !self.can_move_within_bound(board) {
            return false;
        }
        self.slide();
        true
    }

    fn can_move_within_bound(&self, board: &Vec<Vec<u8>>) -> bool {
        let m = board.len();
        let n = board[0].len();
        match self.direction {
            HeadDirection::Up => self.i != 0,
            HeadDirection::Down => self.i < m - 1,
            HeadDirection::Left => self.j != 0,
            HeadDirection::Right => self.j < n - 1,
        }
    }

    fn will_hit_block(&self, board: &Vec<Vec<u8>>) -> bool {
        match self.direction {
            HeadDirection::Up => board[self.i - 1][self.j] == ('#' as u8),
            HeadDirection::Down => board[self.i + 1][self.j] == ('#' as u8),
            HeadDirection::Left => board[self.i][self.j - 1] == ('#' as u8),
            HeadDirection::Right => board[self.i][self.j + 1] == ('#' as u8),
        }
    }

    fn rotate(&mut self) {
        self.direction = match self.direction {
            HeadDirection::Up => HeadDirection::Right,
            HeadDirection::Down => HeadDirection::Left,
            HeadDirection::Left => HeadDirection::Up,
            HeadDirection::Right => HeadDirection::Down,
        }
    }

    fn slide(&mut self) {
        match self.direction {
            HeadDirection::Up => self.i -= 1,
            HeadDirection::Down => self.i += 1,
            HeadDirection::Left => self.j -= 1,
            HeadDirection::Right => self.j += 1,
        }
    }

    fn to_coordinate(&self) -> Coordinate {
        Coordinate {
            i: self.i,
            j: self.j,
        }
    }
}

fn main() {
    let mut board = reading::read();
    let start_state = find_start_pos(&board);

    let mut state = start_state.clone();
    let mut visited: HashSet<Coordinate> = HashSet::new();

    while state.visit_tracked(&board, &mut visited) {}

    let mut valid_blocks: HashSet<Coordinate> = HashSet::new();
    for coordinate in visited.iter() {
        let i = coordinate.i;
        let j = coordinate.j;
        board[i][j] = '#' as u8;
        let mut new_state = start_state.clone();
        if new_state.visit_until_looped(&board, &mut HashSet::new()) {
            valid_blocks.insert(Coordinate { i, j });
        }
        board[i][j] = '.' as u8;
    }
    println!("Result: {}", visited.len());
    println!("Result blocks size: {}", valid_blocks.len());
}

fn find_start_pos(board: &Vec<Vec<u8>>) -> State {
    board
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &data)| data == ('^' as u8))
                .map(move |(j, _)| (i, j))
        })
        .map(|(i, j)| State {
            i,
            j,
            direction: HeadDirection::Up,
        })
        .next()
        .expect("No ^ in input")
}
