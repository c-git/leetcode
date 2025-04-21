//! Solution for https://leetcode.com/problems/count-the-hidden-sequences
//! 2145. Count the Hidden Sequences

impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut curr = 0;
        let mut min = 0;
        let mut max = 0;
        for diff in differences {
            curr += diff;
            min = min.min(curr);
            max = max.max(curr);
        }
        let min_adjustment = lower - min;
        let adj_max = max + min_adjustment;
        std::cmp::max(0, upper - adj_max + 1)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,-3,4], 1, 6, 2)]
    #[case(vec![3,-4,5,1,-2], -4, 5, 4)]
    #[case(vec![4,-7,2], 3, 6, 0)]
    fn case(
        #[case] differences: Vec<i32>,
        #[case] lower: i32,
        #[case] upper: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::number_of_arrays(differences, lower, upper);
        assert_eq!(actual, expected);
    }
}
