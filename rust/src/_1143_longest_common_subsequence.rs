//! Solution for https://leetcode.com/problems/longest-common-subsequence
//! 1143. Longest Common Subsequence

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (text1, text2) = if text1.len() <= text2.len() {
            (text1.as_bytes(), text2.as_bytes())
        } else {
            (text2.as_bytes(), text1.as_bytes())
        };
        // Store longest common subsequence up to each index for a row
        let mut last_row = vec![0; text1.len()];
        let mut curr_row = last_row.clone();

        for c2 in text2.iter() {
            curr_row[0] = if &text1[0] == c2 {
                last_row[0].max(1)
            } else {
                last_row[0]
            };
            for (idx1, c1) in text1.iter().enumerate().skip(1) {
                let best_previous = curr_row[idx1 - 1].max(last_row[idx1]);
                curr_row[idx1] = if c1 == c2 {
                    best_previous.max(last_row[idx1 - 1] + 1)
                } else {
                    best_previous
                };
            }
            std::mem::swap(&mut last_row, &mut curr_row);
        }

        *last_row.last().unwrap()
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
