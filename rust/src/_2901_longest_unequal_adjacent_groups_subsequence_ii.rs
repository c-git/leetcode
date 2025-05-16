//! Solution for https://leetcode.com/problems/longest-unequal-adjacent-groups-subsequence-ii
//! 2901. Longest Unequal Adjacent Groups Subsequence II

impl Solution {
    /// Copied from editorial
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let n = groups.len();

        let mut dp = vec![1; n];
        let mut prev = vec![-1; n];
        let mut max_index = 0;
        for i in 1..n {
            for j in 0..i {
                if Self::check(&words[i], &words[j]) && dp[j] + 1 > dp[i] && groups[i] != groups[j]
                {
                    dp[i] = dp[j] + 1;
                    prev[i] = j as i32;
                }
            }
            if dp[i] > dp[max_index] {
                max_index = i;
            }
        }
        let mut ans = Vec::new();
        let mut i = max_index as i32;
        while i >= 0 {
            ans.push(words[i as usize].clone());
            i = prev[i as usize];
        }
        ans.reverse();
        ans
    }

    fn check(s1: &String, s2: &String) -> bool {
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
