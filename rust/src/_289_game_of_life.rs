//! Solution for https://leetcode.com/problems/game-of-life
//! 289. Game of Life

const DEAD: i32 = 0;
const ALIVE: i32 = 1;
const WAS_ALIVE: i32 = 2; // Will become dead
const WAS_DEAD: i32 = 3; // Will become alive

/// Check if the cell value represents a live cell at time step 0
fn is_alive(x: i32) -> bool {
    x == ALIVE || x == WAS_ALIVE
}

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let row_count = board.len();
        let col_count = board[0].len();
        for row in 0..row_count {
            for col in 0..col_count {
                let mut num_alive_neighbours = 0;
                // Top neighbours
                if row > 0 {
                    // Top left
                    if col > 0 && is_alive(board[row - 1][col - 1]) {
                        num_alive_neighbours += 1;
                    }
                    // Straight Up
                    if is_alive(board[row - 1][col]) {
                        num_alive_neighbours += 1;
                    }
                    // Top right
                    if col < col_count - 1 && is_alive(board[row - 1][col + 1]) {
                        num_alive_neighbours += 1;
                    }
                }

                // Straight left
                if col > 0 && is_alive(board[row][col - 1]) {
                    num_alive_neighbours += 1;
                }

                // Straight right
                if col < col_count - 1 && is_alive(board[row][col + 1]) {
                    num_alive_neighbours += 1;
                }

                // Bottom Neighbours
                if row < row_count - 1 {
                    // Bottom left
                    if col > 0 && is_alive(board[row + 1][col - 1]) {
                        num_alive_neighbours += 1;
                    }
                    // Straight Down
                    if is_alive(board[row + 1][col]) {
                        num_alive_neighbours += 1;
                    }
                    // Bottom right
                    if col < col_count - 1 && is_alive(board[row + 1][col + 1]) {
                        num_alive_neighbours += 1;
                    }
                }

                let is_cell_alive = is_alive(board[row][col]);
                match (is_cell_alive, num_alive_neighbours) {
                    (true, num_alive) if num_alive < 2 => board[row][col] = WAS_ALIVE,
                    (true, 2) | (true, 3) => {} // By rule 2 it survies
                    (true, num_alive) if num_alive > 3 => board[row][col] = WAS_ALIVE,
                    (false, 3) => board[row][col] = WAS_DEAD,
                    _ => {}
                }
            }
        }

        for row_values in board {
            for cell in row_values {
                if cell == &WAS_ALIVE {
                    *cell = DEAD;
                } else if cell == &WAS_DEAD {
                    *cell = ALIVE;
                }
            }
        }
    }
}
