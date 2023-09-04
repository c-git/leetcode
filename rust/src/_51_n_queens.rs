//! Solution for https://leetcode.com/problems/n-queens
//! 51. N-Queens

// Source: https://leetcode.com/problems/n-queens/solutions/3459335/rust-recursive-backtracking-with-bitsets-beats-100/

struct Solver {
    result: Vec<Vec<String>>,
    table: Vec<String>,
    n: i32,
    col_bitset: u32,
    row_bitset: u32,
    diag_sum_bitset: u32,
    diag_sub_bitset: u32,
    count: i32,
}

impl Solver {
    fn new(n: i32) -> Self {
        Solver {
            result: Vec::new(),
            table: vec![".".repeat(n as usize); n as usize],
            n,
            col_bitset: 0,
            row_bitset: 0,
            diag_sum_bitset: 0,
            diag_sub_bitset: 0,
            count: 0,
        }
    }
    fn can_be_queen(&self, x: i32, y: i32) -> bool {
        if self.col_bitset & (1 << x) != 0 {
            return false;
        }
        if self.row_bitset & (1 << y) != 0 {
            return false;
        }
        if self.diag_sum_bitset & (1 << (x + y)) != 0 {
            return false;
        }
        if self.diag_sub_bitset & (1 << (x - y + self.n)) != 0 {
            return false;
        }
        true
    }
    fn set_elem(&mut self, x: i32, y: i32, c: u8) {
        if c == b'.' {
            self.col_bitset &= !(1 << x);
            self.row_bitset &= !(1 << y);
            self.diag_sum_bitset &= !(1 << (x + y));
            self.diag_sub_bitset &= !(1 << (x - y + self.n));
            self.count -= 1;
        } else {
            assert_eq!(c, b'Q');
            self.col_bitset |= (1 << x);
            self.row_bitset |= (1 << y);
            self.diag_sum_bitset |= (1 << (x + y));
            self.diag_sub_bitset |= (1 << (x - y + self.n));
            self.count += 1;
        }
        unsafe { self.table[x as usize].as_mut_vec()[y as usize] = c }
    }
    fn solve(&mut self, x: i32) {
        if x == self.n {
            if self.n == self.count {
                self.result.push(self.table.clone());
            }
            return;
        }

        for y in 0..self.n {
            if !self.can_be_queen(x, y) {
                continue;
            }
            self.set_elem(x, y, b'Q');
            self.solve(x + 1);
            self.set_elem(x, y, b'.');
        }
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
