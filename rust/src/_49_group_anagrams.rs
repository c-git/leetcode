//! Solution for https://leetcode.com/problems/group-anagrams
//! 49. Group Anagrams

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result: HashMap<[u8; 26], Vec<String>> = HashMap::new();
        for s in strs {
            let mut characteristic = [0; 26];
            for c in s.as_bytes() {
                characteristic[(c - b'a') as usize] += 1;
            }
            result.entry(characteristic).or_default().push(s);
        }
        result.into_values().collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec!["eat".into(),"tea".into(),"tan".into(),"ate".into(),"nat".into(),"bat".into()], vec![vec!["bat".into()],vec!["nat".into(),"tan".into()],vec!["ate".into(),"eat".into(),"tea".into()]])]
    #[case(vec!["".into()], vec![vec!["".into()]])]
    #[case(vec!["a".into()], vec![vec!["a".into()]])]
    fn case(#[case] strs: Vec<String>, #[case] mut expected: Vec<Vec<String>>) {
        let mut actual = Solution::group_anagrams(strs);
        actual.iter_mut().for_each(|x| x.sort_unstable());
        actual.sort_unstable();
        expected.iter_mut().for_each(|x| x.sort_unstable());
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }
}
