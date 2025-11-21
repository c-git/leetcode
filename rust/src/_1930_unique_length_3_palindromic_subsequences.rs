//! Solution for https://leetcode.com/problems/unique-length-3-palindromic-subsequences
//! 1930. Unique Length-3 Palindromic Subsequences

use std::collections::HashSet;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let s = s.as_bytes();
        let mut last_instance: [Option<usize>; 26] = [None; 26];
        let mut freq = [0u8; 26];
        let mut middles: Vec<HashSet<u8>> = vec![HashSet::new(); 26];
        for (i, c) in s.iter().enumerate() {
            let char_index = (c - b'a') as usize;
            freq[char_index] += 1;
            if let Some(this_prev) = last_instance[char_index] {
                for other_prev in last_instance.iter().filter_map(|&x| x) {
                    if other_prev > this_prev {
                        middles[char_index].insert(s[other_prev]);
                    }
                }
            }
            last_instance[char_index] = Some(i);
        }

        let three_same_count = freq.iter().filter(|&&x| x >= 3).count();
        let other_middles: usize = middles.iter().map(|x| x.len()).sum();
        (three_same_count + other_middles) as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("aabca", 3)]
    #[case("adc", 0)]
    #[case("bbcbaba", 4)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::count_palindromic_subsequence(s);
        assert_eq!(actual, expected);
    }
}
