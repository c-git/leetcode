//! Solution for https://leetcode.com/problems/longest-common-subsequence
//! 1143. Longest Common Subsequence

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        // Taken from https://leetcode.com/problems/longest-common-subsequence/solutions/2394254/rust-dp-with-comments/

        // Sort string by longer and shorter to reduce the memory usage
        let (shorter, longer) = if text1.len() < text2.len() {
            (text1, text2)
        } else {
            (text2, text1)
        };

        // Input is ASCII => chars are bytes
        let (longer, shorter) = (longer.as_bytes(), shorter.as_bytes());

        // Initialize previous DP row. All zeros represent taking no characters from text1
        let mut dp_prev = vec![0; shorter.len() + 1];
        let mut dp_curr = dp_prev.clone();

        // Iterate in reverse over the text strings, keeping track of the LCS considering the
        // corresponding suffixes
        for idx_longer in (0..longer.len()).rev() {
            for idx_shorter in (0..shorter.len()).rev() {
                // Take the best path - either skipping the current character in shorter, or
                // skipping the current character in longer, or using the characters if they match.
                dp_curr[idx_shorter] = dp_prev[idx_shorter].max(dp_curr[idx_shorter + 1]).max(
                    dp_prev[idx_shorter + 1]
                        + if longer[idx_longer] == shorter[idx_shorter] {
                            1
                        } else {
                            0
                        },
                );
            }
            // Swap the rows to reuse dp_prev, which is now stale
            std::mem::swap(&mut dp_prev, &mut dp_curr);
        }

        dp_prev[0]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("abcde", "ace", 3)]
    #[case("abcde", "cce", 2)]
    #[case("abcde", "ce", 2)]
    #[case("abc", "abc", 3)]
    #[case("abc", "def", 0)]
    #[case("abcba", "abcbcba", 5)]
    #[case("pmjghexybyrgzczy", "hafcdqbgncrcbihkd", 4)]
    fn case(#[case] text1: String, #[case] text2: String, #[case] expected: i32) {
        let actual = Solution::longest_common_subsequence(text1, text2);
        assert_eq!(actual, expected);
    }
}
