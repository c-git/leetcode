//! Solution for https://leetcode.com/problems/interleaving-string
//! 97. Interleaving String

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let mut memo = vec![vec![vec![None; s3.len() + 1]; s2.len() + 1]; s1.len() + 1];
        Self::is_interleave_(&s1, &s2, &s3, &mut memo)
    }

    fn is_interleave_(s1: &str, s2: &str, s3: &str, memo: &mut [Vec<Vec<Option<bool>>>]) -> bool {
        if let Some(result) = memo[s1.len()][s2.len()][s3.len()] {
            return result;
        }

        let result = match (s1.is_empty(), s2.is_empty(), s3.is_empty()) {
            (true, true, true) => true,                  // All empty
            (_, _, true) | (true, true, false) => false, // Either more letters and no output or more output and no letters
            (false, true, false) => {
                s1.chars().next().unwrap() == s3.chars().next().unwrap()
                    && Self::is_interleave_(&s1[1..], s2, &s3[1..], memo)
            }
            (true, false, false) => {
                s2.chars().next().unwrap() == s3.chars().next().unwrap()
                    && Self::is_interleave_(s1, &s2[1..], &s3[1..], memo)
            }
            (false, false, false) => {
                (s1.chars().next().unwrap() == s3.chars().next().unwrap()
                    && Self::is_interleave_(&s1[1..], s2, &s3[1..], memo))
                    || (s2.chars().next().unwrap() == s3.chars().next().unwrap()
                        && Self::is_interleave_(s1, &s2[1..], &s3[1..], memo))
            }
        };

        memo[s1.len()][s2.len()][s3.len()] = Some(result);
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
    #[case("aabcc", "dbbca", "aadbbcbcac", true)]
    #[case("aabcc", "dbbca", "aadbbbaccc", false)]
    #[case("", "", "", true)]
    fn case(#[case] s1: String, #[case] s2: String, #[case] s3: String, #[case] expected: bool) {
        let actual = Solution::is_interleave(s1, s2, s3);
        assert_eq!(actual, expected);
    }
}
