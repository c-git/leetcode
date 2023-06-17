//! Solution for https://leetcode.com/problems/number-of-ways-to-reorder-array-to-get-same-bst
//! 1569. Number of Ways to Reorder Array to Get Same BST

impl Solution {
    const MOD_BASE: u64 = 1_000_000_007;

    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return 0;
        }

        let root = nums[0];
        let mut left = vec![];
        let mut right = vec![];
        let n = nums.len() as u64;
        for num in nums.into_iter().skip(1) {
            debug_assert_ne!(root, num);
            if num < root {
                left.push(num)
            } else {
                right.push(num)
            }
        }

        if left.is_empty() {
            Self::num_of_ways(right)
        } else if right.is_empty() {
            Self::num_of_ways(left)
        } else {
            let mut result = Self::choose(n - 1, left.len() as u64) - 1;
            let left_subtree = Self::num_of_ways(left) as u64;
            let right_subtree = Self::num_of_ways(right) as u64;
            if left_subtree > 0 {
                result = (result * left_subtree) % Self::MOD_BASE;
            }
            if right_subtree > 0 {
                result = (result * right_subtree) % Self::MOD_BASE;
            }
            result as i32
        }
    }

    /// Source: https://en.wikipedia.org/wiki/Combination#Example_of_counting_combinations
    fn choose(n: u64, r: u64) -> u64 {
        debug_assert!(n >= r);
        let mut result = 1;
        for x in 0..r {
            result = ((result * (n - x)) / (x + 1)) % Self::MOD_BASE;
        }
        result
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,1,3], 1)]
    #[case(vec![3,4,5,1,2], 5)]
    #[case(vec![1,2,3], 0)]
    #[case(vec![
            4, 58, 68, 45, 35, 60, 27, 62, 67, 42, 64, 31, 63, 17, 43, 40, 1, 3, 9, 48, 47, 24, 66,
            37, 36, 12, 29, 5, 65, 46, 30, 57, 2, 21, 32, 55, 39, 53, 54, 11, 51, 7, 28, 13, 18,
            61, 34, 16, 59, 10, 23, 14, 33, 49, 22, 56, 15, 25, 50, 41, 20, 38, 69, 19, 26, 6, 8,
        ], 195951021)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::num_of_ways(nums);
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(10, 2, 45)]
    #[case(52, 5, 2598960)]
    #[case(100, 5, 75287520)]
    #[case(200, 5, 535650026)]
    #[case(200, 10, 151856252)]
    fn choose(#[case] n: u64, #[case] r: u64, #[case] expected: u64) {
        dbg!(Solution::MOD_BASE);
        assert_eq!(Solution::choose(n, r), expected);
    }
}
