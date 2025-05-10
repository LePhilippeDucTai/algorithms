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
const BOARD_DIM: usize = DIM * DIM;
const BLOCK_DIM: usize = DIM.isqrt();

type Candidates = HashMap<(usize, usize), HashSet<u8>>;

struct Board {
    board: [[u8; DIM]; DIM],
    candidates: Candidates,
}

fn compute_taken_items_at(i: usize, j: usize, board: &[[u8; DIM]; DIM]) -> HashSet<u8> {
    let row: HashSet<u8> = (0..DIM).map(|x| board[i][x]).collect();
    let col: HashSet<u8> = (0..DIM).map(|x| board[x][j]).collect();
    let block: HashSet<u8> = {
        let i0 = i / BLOCK_DIM;
        let j0 = j / BLOCK_DIM;
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
    let candidates = HashSet::from(CANDIDATES);
    for (i, j) in (0..DIM).cartesian_product(0..DIM) {
        if board[i][j] != NULL_ELEMENT {
            let unions = compute_taken_items_at(i, j, board);
            available_positions.insert((i, j), &candidates - &unions);
        }
    }
    available_positions
}

impl Board {
    fn new(board: [[u8; DIM]; DIM]) -> Self {
        let available_positions = initial_scan(&board);
        Board {
            board,
            candidates: available_positions,
        }
    }

    fn is_solved(&self) -> bool {
        false
    }
}

fn sudoku_solver(board: [[u8; 9]; 9]) -> [[u8; 9]; 9] {
    // Create a mutable copy of the input board
    let mut board = board;

    // Attempt to solve the board
    if solve(&mut board) {
        board
    } else {
        // Return original board if no solution found
        board
    }
}

// Helper function to solve the Sudoku puzzle using backtracking
fn solve(board: &mut [[u8; 9]; 9]) -> bool {
    if let Some((row, col)) = find_empty(&board) {};
    true
}

// Find an empty cell (represented by 0)
fn find_empty(board: &[[u8; 9]; 9]) -> Option<(usize, usize)> {
    (1..9)
        .cartesian_product(1..9)
        .find(|(i, j)| board[*i][*j] == 0)
}

// Check if placing 'num' at position (row, col) is valid
fn is_valid(board: &[[u8; 9]; 9], row: usize, col: usize, num: u8) -> bool {
    true
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
                if board_data[i][j] != NULL_ELEMENT {
                    assert!(board.candidates.contains_key(&(i, j)));
                }
            }
        }
    }
}
