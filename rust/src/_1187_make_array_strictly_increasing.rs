//! Solution for https://leetcode.com/problems/make-array-strictly-increasing
//! 1187. Make Array Strictly Increasing

use std::collections::HashMap;

impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
        // Based on Editorial
        let mut dp = HashMap::new();
        dp.insert(-1, 0);
        arr2.sort();
        let n = arr2.len();

        #[allow(clippy::needless_range_loop)]
        for i in 0..arr1.len() {
            let mut new_dp: HashMap<i32, i32> = HashMap::new();
            for (prev, prev_val) in dp {
                if arr1[i] > prev {
                    new_dp
                        .entry(arr1[i])
                        .and_modify(|x| *x = prev_val.min(*x))
                        .or_insert(prev_val);
                }
                let idx = arr2.partition_point(|&x| x <= prev);
                if idx < n {
                    let candidate_op_count = 1 + prev_val;
                    new_dp
                        .entry(arr2[idx])
                        .and_modify(|x| *x = (*x).min(candidate_op_count))
                        .or_insert(candidate_op_count);
                }
            }
            dp = new_dp;
        }
        dp.into_values().min().unwrap_or(-1)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,5,3,6,7], vec![1,3,2,4], 1)]
    #[case(vec![1,5,3,6,7], vec![4,3,1], 2)]
    #[case(vec![1,5,3,6,7], vec![1,6,3,3], -1)]
    fn case(#[case] arr1: Vec<i32>, #[case] arr2: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::make_array_increasing(arr1, arr2);
        assert_eq!(actual, expected);
    }
}
