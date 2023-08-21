//! Solution for https://leetcode.com/problems/repeated-substring-pattern
//! 459. Repeated Substring Pattern

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let n = s.len();
        'outer: for i in 1..n {
            if n % i != 0 {
                // Not a multiple of i, will not be able to be a solution
                continue;
            }
            let substring = &s[0..i];
            for compare_idx in (i..n).step_by(i) {
                let candidate = &s[compare_idx..i + compare_idx];
                if candidate != substring {
                    continue 'outer;
                }
            }
            return true;
        }
        false
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("abab", true)]
    #[case("aba", false)]
    #[case("abcabcabcabc", true)]
    #[case("ab", false)]
    #[case("aaaaaaa", true)]
    #[case("babbabbabbabbabbabbabbab", true)]
    fn case(#[case] s: String, #[case] expected: bool) {
        let actual = Solution::repeated_substring_pattern(s);
        assert_eq!(actual, expected);
    }
}
