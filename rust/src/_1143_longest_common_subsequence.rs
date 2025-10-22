//! Solution for https://leetcode.com/problems/longest-common-subsequence
//! 1143. Longest Common Subsequence

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1 = text1.as_bytes();
        let text2 = text2.as_bytes();
        let mut prev_row = vec![0; text1.len()];
        let mut curr_row = vec![0; text1.len()];

        for c2 in text2.iter() {
            for (i, c1) in text1.iter().enumerate() {
                curr_row[i] = prev_row[i];
                if i > 0 {
                    curr_row[i] = curr_row[i].max(curr_row[i - 1]);
                }
                if c1 == c2 {
                    let extend_on = if i > 0 { prev_row[i - 1] } else { 0 };
                    curr_row[i] = curr_row[i].max(extend_on + 1);
                }
            }
            std::mem::swap(&mut prev_row, &mut curr_row);
        }
        prev_row
            .iter()
            .max()
            .copied()
            .expect("should have at least one char")
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
    #[case("jghbrgc", "hcbgcrcbhk", 4)]
    fn case(#[case] text1: String, #[case] text2: String, #[case] expected: i32) {
        let actual = Solution::longest_common_subsequence(text1, text2);
        assert_eq!(actual, expected);
    }
}
