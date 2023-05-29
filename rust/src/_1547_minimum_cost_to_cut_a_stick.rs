use std::cmp::min;

impl Solution {
    pub fn min_cost(n: i32, mut cuts: Vec<i32>) -> i32 {
        // After reading editorial
        cuts.push(0);
        cuts.push(n);
        cuts.sort_unstable();
        let m = cuts.len();

        // TODO Add what each cell in the matrix represents
        let mut dp = vec![vec![0; m]; m];

        for diff in 2..m {
            for left in 0..m - diff {
                let right = left + diff;
                let mut ans = i32::MAX;
                for mid in left + 1..right {
                    ans = min(
                        ans,
                        dp[left][mid] + dp[mid][right] + cuts[right] - cuts[left],
                    );
                }
                dp[left][right] = ans;
            }
        }

        dp[0][m - 1]
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(7,vec![1,3,4,5],16)]
    #[case(9,vec![5,6,1,4,2],22)]
    fn case(#[case] n: i32, #[case] cuts: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_cost(n, cuts);
        assert_eq!(actual, expected);
    }
}
