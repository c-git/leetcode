//! Solution for https://leetcode.com/problems/minimum-operations-to-make-array-values-equal-to-k
//! 3375. Minimum Operations to Make Array Values Equal to K

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        if nums.first().unwrap() < &k {
            return -1;
        }
        let mut last_num = *nums.last().unwrap();
        let mut result = 0;
        for num in nums.iter().cloned().rev() {
            if num < last_num {
                last_num = num;
                result += 1;
            }
        }
        if last_num > k {
            result += 1;
        }
        result
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![5,2,5,4,5], 2, 2)]
    #[case(vec![2,1,2], 2, -1)]
    #[case(vec![9,7,5,3], 1, 4)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::min_operations(nums, k);
        assert_eq!(actual, expected);
    }
}
