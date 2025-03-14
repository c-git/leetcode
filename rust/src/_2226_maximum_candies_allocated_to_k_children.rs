//! Solution for https://leetcode.com/problems/maximum-candies-allocated-to-k-children
//! 2226. Maximum Candies Allocated to K Children

impl Solution {
    pub fn maximum_candies(mut candies: Vec<i32>, k: i64) -> i32 {
        let mut result = 0;
        candies.sort_unstable();
        let mut candies_sum = 0;
        for curr_count in candies.iter().map(|x| *x as i64).rev() {
            candies_sum += curr_count;
            let per_child = (candies_sum / k).clamp(0, curr_count);
            result = result.max(per_child);
        }
        result as i32
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
    fn case(#[case] candies: Vec<i32>, #[case] k: i64, #[case] expected: i32) {
        let actual = Solution::maximum_candies(candies, k);
        assert_eq!(actual, expected);
    }
}
