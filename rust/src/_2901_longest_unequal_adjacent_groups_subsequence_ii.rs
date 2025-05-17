//! Solution for https://leetcode.com/problems/longest-unequal-adjacent-groups-subsequence-ii
//! 2901. Longest Unequal Adjacent Groups Subsequence II

impl Solution {
    /// Based on editorial
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let n = groups.len();
        // dp[i] = (Previous, Longest sequence ending at i)
        let mut dp = vec![(None, 1); n];
        let mut max_index = 0;
        for i in 1..n {
            for prev in 0..i {
                if Self::check(&words[i], &words[prev])
                    && dp[prev].1 + 1 > dp[i].1
                    && groups[i] != groups[prev]
                {
                    dp[i] = (Some(prev), dp[prev].1 + 1);
                }
            }
            if dp[i].1 > dp[max_index].1 {
                max_index = i;
            }
        }
        let mut result = Vec::new();
        let mut prev_opt = Some(max_index);
        while let Some(i) = prev_opt {
            result.push(words[i].clone());
            prev_opt = dp[i].0;
        }
        result.reverse();
        result
    }

    fn check(s1: &str, s2: &str) -> bool {
        if s1.len() != s2.len() {
            return false;
        }
        let mut diff = 0;
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 != c2 {
                diff += 1;
                if diff > 1 {
                    return false;
                }
            }
        }
        diff == 1
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec!["bab".into(),"dab".into(),"cab".into()], vec![1,2,2], vec!["bab".into(), "dab".into()])]
    #[case(vec!["a".into(),"b".into(),"c".into(),"d".into()], vec![1,2,3,4], vec!["a".into(),"b".into(),"c".into(),"d".into()])]
    fn case(#[case] words: Vec<String>, #[case] groups: Vec<i32>, #[case] expected: Vec<String>) {
        let actual = Solution::get_words_in_longest_subsequence(words, groups);
        assert_eq!(actual, expected);
    }
}
