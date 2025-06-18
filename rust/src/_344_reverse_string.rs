//! Solution for https://leetcode.com/problems/reverse-string
//! 344. Reverse String

impl Solution {
    pub fn reverse_string(s: &mut [char]) {
        if s.len() <= 1 {
            return;
        }
        let n = s.len();
        s.swap(0, n - 1);
        Self::reverse_string(&mut s[1..=n - 2]);
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec!['h','e','l','l','o'])]
    #[case(vec!['H','a','n','n','a','h'])]
    fn case(#[case] mut s: Vec<char>) {
        let mut expected = s.clone();
        expected.reverse();
        Solution::reverse_string(&mut s);
        let actual = s;
        assert_eq!(actual, expected);
    }
}
