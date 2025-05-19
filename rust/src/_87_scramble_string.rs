//! Solution for https://leetcode.com/problems/scramble-string
//! 87. Scramble String

impl Solution {
    #[expect(clippy::needless_range_loop)]
    /// Based on Editorial
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let n = s1.len();

        // For each given dp state, we need 3 variables: length, i, and j.
        //
        // Each state will focus on two substrings. The first one will be a substring of
        // s1, starting at index i with length equal to length - let's call this
        // substring s. The second one will be a substring of s2, starting at index j
        // with length - let's call this substring t.
        //
        // Let dp[length][i][j] be a boolean representing whether t is a scrambled
        // version of s.
        let mut dp = vec![vec![vec![false; n]; n]; n + 1];
        for i in 0..n {
            for j in 0..n {
                dp[1][i][j] = s1[i] == s2[j];
            }
        }
        for length in 2..=n {
            for i in 0..=(n - length) {
                for j in 0..=(n - length) {
                    for new_length in 1..length {
                        let dp1 = dp[new_length][i].clone();
                        let dp2 = dp[length - new_length][i + new_length].clone();
                        dp[length][i][j] |= dp1[j] && dp2[j + new_length];
                        dp[length][i][j] |= dp1[j + length - new_length] && dp2[j];
                    }
                }
            }
        }

        dp[n][0][0]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("great", "rgeat", true)]
    #[case("abcde", "caebd", false)]
    #[case("a", "a", true)]
    #[case("abcde", "cabed", true)]
    #[case("abcdbdacbdac", "bdacabcdbdac", true)]
    fn case(#[case] s1: String, #[case] s2: String, #[case] expected: bool) {
        let actual = Solution::is_scramble(s1, s2);
        assert_eq!(actual, expected);
    }
}
