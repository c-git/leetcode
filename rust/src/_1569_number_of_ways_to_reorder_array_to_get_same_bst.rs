//! Solution for https://leetcode.com/problems/number-of-ways-to-reorder-array-to-get-same-bst
//! 1569. Number of Ways to Reorder Array to Get Same BST

impl Solution {
    const MOD_BASE: u128 = 1_000_000_007;

    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return 0;
        }

        let root = nums[0];
        let mut left = vec![];
        let mut right = vec![];
        let n = nums.len() as i32;
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
            let mut result = Self::choose(n - 1, left.len() as i32) - 1;
            let left_subtree = Self::num_of_ways(left) as u128;
            let right_subtree = Self::num_of_ways(right) as u128;
            if left_subtree > 0 {
                result = (result * left_subtree) % Self::MOD_BASE;
            }
            if right_subtree > 0 {
                result = (result * right_subtree) % Self::MOD_BASE;
            }
            result as i32
        }
    }

    fn choose(n: i32, r: i32) -> u128 {
        let n = n as u128;
        let r = r as u128;
        Self::factorial(n) / (Self::factorial(r) * Self::factorial(n - r))
    }

    fn factorial(n: u128) -> u128 {
        let mut result = 1;
        for i in 2..=n {
            result *= i;
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
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::num_of_ways(nums);
        assert_eq!(actual, expected);
    }
}
