//! Solution for https://leetcode.com/problems/construct-the-minimum-bitwise-array-i
//! 3314. Construct the Minimum Bitwise Array I

impl Solution {
    /// After reading the editorial and watching a few youtube videos that only
    /// show the brute force solution, I've decided to just implement a brute
    /// force solution.
    pub fn min_bitwise_array(mut nums: Vec<i32>) -> Vec<i32> {
        for num in nums.iter_mut() {
            let target = *num; // Copy target value
            *num = -1;
            for candidate in 0..target {
                if (candidate | (candidate + 1)) == target {
                    *num = candidate;
                    break;
                }
            }
        }
        nums
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,3,5,7], vec![-1,1,4,3])]
    #[case(vec![11,13,31], vec![9,12,15])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::min_bitwise_array(nums);
        assert_eq!(actual, expected);
    }
}
