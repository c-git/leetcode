//! Solution for https://leetcode.com/problems/merge-strings-alternately
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::with_capacity(word1.len() + word2.len());
        let mut word1 = word1.chars();
        let mut word2 = word2.chars();
        #[allow(clippy::while_let_loop)]
        loop {
            if let Some(c1) = word1.next() {
                result.push(c1);
            } else {
                break;
            }
            if let Some(c2) = word2.next() {
                result.push(c2);
            } else {
                break;
            }
        }
        for c in word1 {
            result.push(c);
        }
        for c in word2 {
            result.push(c);
        }
        result
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("abc", "pqr", "apbqcr")]
    #[case("ab", "pqrs", "apbqrs")]
    #[case("abcd", "pq", "apbqcd")]
    fn case(#[case] word1: String, #[case] word2: String, #[case] expected: String) {
        let actual = Solution::merge_alternately(word1, word2);
        assert_eq!(actual, expected);
    }
}
