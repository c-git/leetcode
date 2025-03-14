//! Solution for https://leetcode.com/problems/maximum-candies-allocated-to-k-children
//! 2226. Maximum Candies Allocated to K Children

impl Solution {
    /// After reading hints from leetcode
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let mut low = 1;
        if !Self::is_possible(&candies, low, k) {
            return 0;
        }
        let Some(mut high) = candies.iter().max().copied() else {
            // No candies to share
            return 0;
        };
        if Self::is_possible(&candies, high, k) {
            return k as i32;
        }
        while low < high {
            debug_assert!(Self::is_possible(&candies, low, k));
            debug_assert!(!Self::is_possible(&candies, high, k));
            let mid = (low + high) / 2;
            if mid == low {
                return low;
            }
            if Self::is_possible(&candies, mid, k) {
                low = mid;
            } else {
                high = mid;
            }
        }

        low
    }

    fn is_possible(candies: &[i32], per_child: i32, k: i64) -> bool {
        k <= candies.iter().map(|x| (x / per_child) as i64).sum::<i64>()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![5,8,6], 3, 5)]
    #[case(vec![2,5], 11, 0)]
    #[case(vec![5,12,6], 3, 6)]
    #[case(vec![5,60,6], 3, 20)]
    #[case(vec![2,3,4], 3, 2)]
    #[case(vec![2,4,7,5], 4, 3)]
    #[case(vec![4,12], 4, 4)]
    fn case(#[case] candies: Vec<i32>, #[case] k: i64, #[case] expected: i32) {
        let actual = Solution::maximum_candies(candies, k);
        assert_eq!(actual, expected);
    }
}
