//! Solution for https://leetcode.com/problems/n-queens-ii
//! 52. N-Queens II

// Based on solution to 51. N-Queens

enum ElementType {
    Queen,
    Empty,
}

struct Solver {
    result: i32,
    n: i32,
    col_bitset: u32,
    diag_sum_bitset: u32,
    diag_sub_bitset: u32,
    placed_queen_count: i32,
}

impl Solver {
    fn new(n: i32) -> Self {
        Solver {
            result: 0,
            n,
            col_bitset: 0,
            diag_sum_bitset: 0,
            diag_sub_bitset: 0,
            placed_queen_count: 0,
        }
    }

    fn can_be_queen(&self, row: i32, col: i32) -> bool {
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

    fn set_elem(&mut self, row: i32, col: i32, element_type: ElementType) {
        match element_type {
            ElementType::Queen => {
                self.col_bitset |= 1 << col;
                self.diag_sum_bitset |= 1 << (row + col);
                self.diag_sub_bitset |= 1 << (row - col + self.n);
                self.placed_queen_count += 1;
            }
            ElementType::Empty => {
                self.col_bitset &= !(1 << col);
                self.diag_sum_bitset &= !(1 << (row + col));
                self.diag_sub_bitset &= !(1 << (row - col + self.n));
                self.placed_queen_count -= 1;
            }
        }
    }

    fn solve(&mut self, row: i32) {
        // self.print_board();
        if row == self.n {
            if self.n == self.placed_queen_count {
                self.result += 1;
            }
            return;
        }

        for col in 0..self.n {
            if !self.can_be_queen(row, col) {
                continue;
            }
            self.set_elem(row, col, ElementType::Queen);
            self.solve(row + 1);
            self.set_elem(row, col, ElementType::Empty);
        }
    }
}

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
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
    #[case(1, 1)]
    #[case(2, 0)]
    #[case(3, 0)]
    #[case(4, 2)]
    #[case(5, 10)]
    #[case(6, 4)]
    #[case(7, 40)]
    #[case(8, 92)]
    #[case(9, 352)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::total_n_queens(n);
        assert_eq!(actual, expected);
    }
}
