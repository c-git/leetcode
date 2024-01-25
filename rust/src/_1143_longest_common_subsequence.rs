//! Solution for https://leetcode.com/problems/longest-common-subsequence
//! 1143. Longest Common Subsequence

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        // It is safe to use byte values because both are only only ascii chars as per problem constraints
        #[cfg(debug_assertions)]
        println!("Text1: {text1}\nText2: {text2}");
        debug_assert_eq!(text1.len(), text1.as_bytes().len());
        debug_assert_eq!(text2.len(), text2.as_bytes().len());
        let text1 = text1.as_bytes();
        let text2 = text2.as_bytes();

        // Stores the longest solution to that index in each string
        let mut dp = vec![vec![0; text2.len()]; text1.len()];

        #[allow(clippy::needless_range_loop)]
        for index1 in 0..text1.len() {
            let mut has_matched = false;
            for index2 in 0..text2.len() {
                let is_match = text1[index1] == text2[index2];
                let should_increment = is_match && !has_matched;
                dp[index1][index2] = match (index1, index2) {
                    (0, 0) => {
                        // If match set to 1 otherwise no
                        if should_increment {
                            1
                        } else {
                            0
                        }
                    }
                    (0, _) => dp[index1][index2 - 1] + if should_increment { 1 } else { 0 },
                    (_, 0) => {
                        if should_increment {
                            1
                        } else {
                            dp[index1 - 1][index2]
                        }
                    }
                    _ => {
                        if should_increment {
                            dp[index1][index2 - 1] + 1
                        } else {
                            dp[index1 - 1][index2].max(dp[index1][index2 - 1])
                        }
                    }
                };
                has_matched = has_matched || is_match;
                #[cfg(debug_assertions)]
                print_table(index1, index2, has_matched, &dp);
            }
        }

        *dp.last().unwrap().last().unwrap()
    }
}

#[cfg(debug_assertions)]
fn print_table(index1: usize, index2: usize, has_matched: bool, dp: &[Vec<i32>]) {
    println!(
        "({index1},{index2}) - {}",
        if has_matched {
            "matched"
        } else {
            "not matched yet"
        }
    );
    for row in dp {
        println!("{row:?}");
    }
    println!();
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
    fn case(#[case] text1: String, #[case] text2: String, #[case] expected: i32) {
        let actual = Solution::longest_common_subsequence(text1, text2);
        assert_eq!(actual, expected);
    }
}
