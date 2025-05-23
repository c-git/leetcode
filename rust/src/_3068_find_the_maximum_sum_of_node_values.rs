//! Solution for https://leetcode.com/problems/find-the-maximum-sum-of-node-values
//! 3068. Find the Maximum Sum of Node Values

impl Solution {
    /// Translated from editorial
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, _edges: Vec<Vec<i32>>) -> i64 {
        let mut sum_val = 0i64;
        let mut count = 0;
        let mut positive_minimum = i32::MAX;
        let mut negative_maximum = i32::MIN;

        for node_value in nums {
            let operated_node_value = node_value ^ k;
            sum_val += node_value as i64;
            let net_change = operated_node_value - node_value;

            if net_change > 0 {
                positive_minimum = positive_minimum.min(net_change);
                sum_val += net_change as i64;
                count += 1;
            } else {
                negative_maximum = negative_maximum.max(net_change);
            }
        }

        // If the number of positive netChange values is even, return the sum.
        if count % 2 == 0 {
            sum_val
        } else {
            // Otherwise return the maximum of both discussed cases.
            std::cmp::max(
                sum_val - positive_minimum as i64,
                sum_val + negative_maximum as i64,
            )
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,1], 3, vec![vec![0,1],vec![0,2]], 6)]
    #[case(vec![2,3], 7, vec![vec![0,1]], 9)]
    #[case(vec![7,7,7,7,7,7], 3, vec![vec![0,1],vec![0,2],vec![0,3],vec![0,4],vec![0,5]], 42)]
    #[case(vec![24,78,1,97,44], 6, vec![], 260)] //edges not copied
    #[case(vec![3,45,1,27,87,43,62], 8, vec![], 284)] //edges not copied
    fn case(
        #[case] nums: Vec<i32>,
        #[case] k: i32,
        #[case] edges: Vec<Vec<i32>>,
        #[case] expected: i64,
    ) {
        let actual = Solution::maximum_value_sum(nums, k, edges);
        assert_eq!(actual, expected);
    }
}
