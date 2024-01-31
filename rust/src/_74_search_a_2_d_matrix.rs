//! Solution for https://leetcode.com/problems/search-a-2d-matrix
//! 74. Search a 2D Matrix

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut low_index = 0;
        let mut high_index = matrix.len() * matrix[0].len() - 1;
        while low_index < high_index {
            // LI: If the value exists it is in the range low_index..=high_index
            let mid = (low_index + high_index) / 2 + 1;
            match matrix.at_adjusted_index(mid).cmp(&target) {
                std::cmp::Ordering::Less => low_index = mid + 1,
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater => high_index = mid - 1,
            }
        }
        // If low goes above high then we know that it could not be found and that way we never underflow
        low_index <= high_index && matrix.at_adjusted_index(low_index) == target
    }
}

trait FlexibleIndex {
    fn at_adjusted_index(&self, index: usize) -> i32;
}

impl FlexibleIndex for Vec<Vec<i32>> {
    fn at_adjusted_index(&self, index: usize) -> i32 {
        let row = index / self[0].len();
        let col = index - row * self[0].len();
        self[row][col]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 3, true)]
    #[case(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 13, false)]
    #[case(vec![vec![1,1]], 2, false)]
    #[case(vec![vec![1,1]], 0, false)]
    fn case(#[case] matrix: Vec<Vec<i32>>, #[case] target: i32, #[case] expected: bool) {
        let actual = Solution::search_matrix(matrix, target);
        assert_eq!(actual, expected);
    }
}
