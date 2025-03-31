fn square(x: f32) -> f32 {
    x * x
}

struct Board([[u8; 9]; 9]);

impl Board {
    fn is_solved(&self) -> bool {
        true
    }
}

fn sudoku_solver(board: [[u8; 9]; 9]) -> [[u8; 9]; 9] {
    board
}

#[cfg(test)]
mod tests {}
