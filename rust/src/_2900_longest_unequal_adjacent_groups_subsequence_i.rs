//! Solution for https://leetcode.com/problems/longest-unequal-adjacent-groups-subsequence-i
//! 2900. Longest Unequal Adjacent Groups Subsequence I

impl Solution {
    pub fn get_longest_subsequence(mut words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut last_group = if groups[0] == 0 { 1 } else { 0 };
        let mut index = 0;
        words.retain(|_| {
            let result = last_group != groups[index];
            last_group = groups[index];
            index += 1;
            result
        });
        words
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec!["c".into()], vec![0], vec!["c".into()])]
    #[case(vec!["d".into()], vec![1], vec!["d".into()])]
    fn case(#[case] words: Vec<String>, #[case] groups: Vec<i32>, #[case] expected: Vec<String>) {
        let actual = Solution::get_longest_subsequence(words, groups);
        assert_eq!(actual, expected);
    }
}
