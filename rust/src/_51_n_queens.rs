//! Solution for https://leetcode.com/problems/n-queens
//! 51. N-Queens

// Source: https://leetcode.com/problems/n-queens/solutions/3459335/rust-recursive-backtracking-with-bitsets-beats-100/
// Modified slightly for ease of readability

struct Solver {
    result: Vec<Vec<String>>,
    board: Vec<String>,
    n: i32,
    row_bitset: u32,
    col_bitset: u32,
    diag_sum_bitset: u32,
    diag_sub_bitset: u32,
    count: i32,
}

impl Solver {
    fn new(n: i32) -> Self {
        Solver {
            result: Vec::new(),
            board: vec![".".repeat(n as usize); n as usize],
            n,
            row_bitset: 0,
            col_bitset: 0,
            diag_sum_bitset: 0,
            diag_sub_bitset: 0,
            count: 0,
        }
    }
    fn can_be_queen(&self, row: i32, col: i32) -> bool {
        if self.row_bitset & (1 << row) != 0 {
            return false;
        }
        if self.col_bitset & (1 << col) != 0 {
            return false;
        }
        if self.diag_sum_bitset & (1 << (row + col)) != 0 {
            return false;
        }
        if self.diag_sub_bitset & (1 << (row - col + self.n)) != 0 {
            return false;
        }
        true
    }
    fn set_elem(&mut self, row: i32, col: i32, c: u8) {
        if c == b'.' {
            self.row_bitset &= !(1 << row);
            self.col_bitset &= !(1 << col);
            self.diag_sum_bitset &= !(1 << (row + col));
            self.diag_sub_bitset &= !(1 << (row - col + self.n));
            self.count -= 1;
        } else {
            debug_assert_eq!(c, b'Q');
            self.row_bitset |= 1 << row;
            self.col_bitset |= 1 << col;
            self.diag_sum_bitset |= 1 << (row + col);
            self.diag_sub_bitset |= 1 << (row - col + self.n);
            self.count += 1;
        }
        unsafe { self.board[row as usize].as_mut_vec()[col as usize] = c }
    }
    fn solve(&mut self, row: i32) {
        // self.print_board();
        if row == self.n {
            if self.n == self.count {
                self.result.push(self.board.clone());
            }
            return;
        }

        for col in 0..self.n {
            if !self.can_be_queen(row, col) {
                continue;
            }
            self.set_elem(row, col, b'Q');
            self.solve(row + 1);
            self.set_elem(row, col, b'.');
        }
    }

    fn print_board(&self) {
        for row in self.board.iter() {
            println!("{row}");
        }
        println!();
        println!("row {:b}", self.row_bitset);
        println!("col {:b}", self.col_bitset);
        println!("sum {:b}", self.diag_sum_bitset);
        println!("sub {:b}", self.diag_sub_bitset);
        println!("cnt {:b}", self.count);
        println!("{}", "-".repeat(80));
    }
}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut solver = Solver::new(n);
        solver.solve(0);
        solver.result
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(4, vec![vec![".Q..".into(),"...Q".into(),"Q...".into(),"..Q.".into()],vec!["..Q.".into(),"Q...".into(),"...Q".into(),".Q..".into()]])]
    #[case(1, vec![vec!["Q".into()]])]
    fn case(#[case] n: i32, #[case] mut expected: Vec<Vec<String>>) {
        let mut actual = Solution::solve_n_queens(n);
        expected.sort();
        actual.sort();
        assert_eq!(actual, expected);
    }
}
