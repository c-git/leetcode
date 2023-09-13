//! Solution for https://leetcode.com/problems/candy
//! 135. Candy

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        // From: https://leetcode.com/problems/candy/solutions/4037646/99-20-greedy-two-one-pass/
        let n = ratings.len();
        let mut candies = vec![1; n];

        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1;
            }
        }

        for i in (0..n - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                candies[i] = std::cmp::max(candies[i], candies[i + 1] + 1);
            }
        }

        candies.iter().sum()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,0,2], 5)]
    #[case(vec![1,2,2], 4)]
    fn case(#[case] ratings: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::candy(ratings);
        assert_eq!(actual, expected);
    }
}
