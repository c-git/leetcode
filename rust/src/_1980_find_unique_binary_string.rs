//! Solution for https://leetcode.com/problems/find-unique-binary-string
//! 1980. Find Unique Binary String

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        // Because nums.len() is so small brute force should be enough
        let mut result = vec!['0'; nums.len()];
        let nums: Vec<_> = nums
            .into_iter()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect();
        while nums.contains(&result) {
            for c in result.iter_mut().rev() {
                if c == &'0' {
                    *c = '1';
                    break;
                } else {
                    *c = '0';
                }
            }
        }
        result.into_iter().collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec!["01".into(),"10".into()])]
    #[case(vec!["00".into(),"01".into()])]
    #[case(vec!["111".into(),"011".into(),"001".into()])]
    fn case(#[case] nums: Vec<String>) {
        let actual = Solution::find_different_binary_string(nums.clone());
        if let Some(first) = nums.first() {
            assert_eq!(actual.len(), first.len());
        }
        assert!(!nums.contains(&actual));
        assert!(actual.chars().all(|c| c == '0' || c == '1'))
    }
}
