//! Solution for https://leetcode.com/problems/longest-common-subsequence
//! 1143. Longest Common Subsequence

enum DidTake {
    Taken,
    Skipped,
}

impl DidTake {
    fn val(&self) -> usize {
        match self {
            DidTake::Taken => 0,
            DidTake::Skipped => 1,
        }
    }
}

trait Helpers {
    fn larger(&self) -> i32;
    fn taken(&self) -> i32;
    fn skipped(&self) -> i32;
}

impl Helpers for [i32; 2] {
    fn larger(&self) -> i32 {
        *self.iter().max().expect("Has 2 values")
    }

    fn taken(&self) -> i32 {
        self[DidTake::Taken.val()]
    }

    fn skipped(&self) -> i32 {
        self[DidTake::Skipped.val()]
    }
}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        // It is safe to use byte values because both are only only ascii chars as per problem constraints
        debug_assert_eq!(text1.len(), text1.as_bytes().len());
        debug_assert_eq!(text2.len(), text2.as_bytes().len());
        let text1 = text1.as_bytes();
        let text2 = text2.as_bytes();

        // Stores the longest solution to that index in each string both (including the char from text2, and without the char from text2)
        // So dp[0][1][0] is the best you can do up to index 0 from text 1 with up to index 1 from text 2 if char at text2 is included and dp[0][1][1] is without the char from text 2
        //Alternatively space could just be allocated and not initialized but I think this was easier to read
        let mut dp = vec![vec![[0, 0]; text2.len()]; text1.len()];

        #[allow(clippy::needless_range_loop)]
        for index1 in 0..text1.len() {
            for index2 in 0..text2.len() {
                let is_char_matching = text1[index1] == text2[index2];
                print_table(index1, index2, &dp);
                dp[index1][index2] = match (index1, index2) {
                    (0, 0) => {
                        // If match set to 1 otherwise no
                        if is_char_matching {
                            [1, 0]
                        } else {
                            [0, 0]
                        }
                    }
                    (0, _) => {
                        if is_char_matching {
                            [
                                dp[index1][index2 - 1].skipped() + 1,
                                dp[index1][index2 - 1].larger(),
                            ]
                        } else {
                            dp[index1][index2 - 1]
                        }
                    }
                    (_, 0) => {
                        if is_char_matching {
                            [1, 0]
                        } else {
                            [0, dp[index1 - 1][index2].taken()]
                        }
                    }
                    _ => {
                        if is_char_matching {
                            [
                                dp[index1][index2 - 1].skipped() + 1,
                                dp[index1][index2 - 1].larger(),
                            ]
                        } else {
                            let best = dp[index1][index2 - 1]
                                .larger()
                                .max(dp[index1 - 1][index2].larger());
                            [best, best]
                        }
                    }
                }
            }
        }

        *dp.last().unwrap().last().unwrap().iter().max().unwrap()
    }
}

fn print_table(index1: usize, index2: usize, dp: &[Vec<[i32; 2]>]) {
    println!("({index1},{index2})");
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
