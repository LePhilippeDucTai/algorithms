use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const DIM: usize = 9;
const NULL_ELEMENT: u8 = 0;

const fn generate_candidates() -> [u8; DIM] {
    let mut arr = [0u8; DIM];
    let mut i = 0;
    while i < DIM {
        arr[i] = (i + 1) as u8;
        i += 1;
    }
    arr
}
const CANDIDATES: [u8; DIM] = generate_candidates();
const BLOCK_DIM: usize = DIM.isqrt();

type Candidates = HashMap<(usize, usize), HashSet<u8>>;

pub struct Board {
    board: [[u8; DIM]; DIM],
    nums: HashSet<u8>,
    candidates: Candidates,
}

fn compute_taken_items_at(i: usize, j: usize, board: &[[u8; DIM]; DIM]) -> HashSet<u8> {
    let row: HashSet<u8> = (0..DIM).map(|x| board[i][x]).collect();
    let col: HashSet<u8> = (0..DIM).map(|x| board[x][j]).collect();
    let block: HashSet<u8> = {
        let i0 = BLOCK_DIM * (i / BLOCK_DIM);
        let j0 = BLOCK_DIM * (j / BLOCK_DIM);
        (i0..(i0 + BLOCK_DIM))
            .cartesian_product(j0..(j0 + BLOCK_DIM))
            .map(|(k, l)| board[k][l])
            .collect()
    };
    let unions: HashSet<u8> = &row | &(&col | &block);
    unions
}
fn initial_scan(board: &[[u8; DIM]; DIM]) -> Candidates {
    let mut available_positions: Candidates = HashMap::new();
    let nums = HashSet::from(CANDIDATES);
    (0..DIM)
        .cartesian_product(0..DIM)
        .filter(|(i, j)| board[*i][*j] == NULL_ELEMENT)
        .for_each(|(i, j)| {
            let unions = compute_taken_items_at(i, j, board);
            available_positions.insert((i, j), &nums - &unions);
        });
    available_positions
}

fn update_candidates(board: &mut Board) {
    let mut available_positions: Candidates = HashMap::new();
    board.candidates.keys().for_each(|(i, j)| {
        let unions = compute_taken_items_at(*i, *j, &board.board);
        available_positions.insert((*i, *j), &board.nums - &unions);
    });
    board.candidates = available_positions;
}

fn easy_fill(board: &mut Board) -> bool {
    let mut has_filled = false;
    board
        .candidates
        .clone()
        .into_iter()
        .filter(|(_, value)| value.len() == 1)
        .for_each(|((i, j), set_val)| {
            let &value = set_val.iter().next().unwrap();
            board.board[i][j] = value;
            board.candidates.remove(&(i, j));
            has_filled = true;
            println!("Filled ({i},{j}) with {value}.")
        });
    has_filled
}
impl Board {
    pub fn new(board: [[u8; DIM]; DIM]) -> Self {
        let candidates = initial_scan(&board);
        let nums = HashSet::from(CANDIDATES);
        Board {
            board,
            nums,
            candidates,
        }
    }

    pub fn solve(&mut self) {
        while easy_fill(self) {
            update_candidates(self);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_new_initializes_candidates() {
        let board_data = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];

        let board = Board::new(board_data);

        // Check that the board is stored correctly
        assert_eq!(board.board, board_data);

        // Check that candidates are initialized for filled cells
        for i in 0..DIM {
            for j in 0..DIM {
                if board_data[i][j] == NULL_ELEMENT {
                    assert!(board.candidates.contains_key(&(i, j)));
                }
            }
        }
    }

    #[test]
    fn test_solve_easy_sudoku() {
        let board_data = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];

        let expected_solution = [
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9],
        ];

        let mut board = Board::new(board_data);
        board.solve();

        assert_eq!(board.board, expected_solution);
    }
}
