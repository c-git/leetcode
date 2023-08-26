//! Solution for https://leetcode.com/problems/maximum-length-of-pair-chain
//! 646. Maximum Length of Pair Chain

impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        // Sort pairs by ending time so that if a pair is able to join to another one then the other one comes before it
        pairs.sort_by_key(|x| x[1]);
        let mut result = 1;
        let mut dp = vec![0; pairs.len()];
        debug_assert!(
            !pairs.is_empty(),
            "Constraints says that there is at least one element"
        );
        dp[0] = 1;
        for (i, pair) in pairs.iter().enumerate().skip(1) {
            let mut length = 0;
            let start = pair[0];
            for prev in 0..i {
                if pairs[prev][1] < start {
                    length = length.max(dp[prev] + 1);
                }
            }
            dp[i] = length;
            result = result.max(length);
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
    #[case(vec![vec![1,2],vec![2,3],vec![3,4]], 2)]
    #[case(vec![vec![1,2],vec![7,8],vec![4,5]], 3)]
    #[case(vec![vec![1,2]], 1)]
    fn case(#[case] pairs: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::find_longest_chain(pairs);
        assert_eq!(actual, expected);
    }
}
