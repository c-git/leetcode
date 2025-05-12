//! Solution for https://leetcode.com/problems/finding-3-digit-even-numbers
//! 2094. Finding 3-Digit Even Numbers

use std::collections::HashSet;

impl Solution {
    /// From editorial
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut nums = HashSet::new();
        let n = digits.len();
        // Traverse the indices of three digits
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    // Determine whether it meets the condition of the target even number
                    if i == j || j == k || i == k {
                        continue;
                    }
                    let num = digits[i] * 100 + digits[j] * 10 + digits[k];
                    if num >= 100 && num % 2 == 0 {
                        nums.insert(num);
                    }
                }
            }
        }
        // Converted to an array sorted in ascending order
        let mut res: Vec<i32> = nums.into_iter().collect();
        res.sort();
        res
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,1,3,0], vec![102,120,130,132,210,230,302,310,312,320])]
    #[case(vec![2,2,8,8,2], vec![222,228,282,288,822,828,882])]
    #[case(vec![3,7,5], vec![])]
    fn case(#[case] digits: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::find_even_numbers(digits);
        assert_eq!(actual, expected);
    }
}
