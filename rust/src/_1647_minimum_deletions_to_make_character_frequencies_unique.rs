//! Solution for https://leetcode.com/problems/minimum-deletions-to-make-character-frequencies-unique
//! 1647. Minimum Deletions to Make Character Frequencies Unique

use std::collections::BTreeMap;

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut result = 0;

        // Count frequencies
        let mut freq_count = [0; 26];
        for b in s.as_bytes() {
            let idx = (b - b'a') as usize; // Only lower case english letters expected
            freq_count[idx] += 1;
        }

        // Get number of items with the same frequency
        let mut freq_dist = BTreeMap::new();
        for count in freq_count {
            if count > 0 {
                freq_dist.entry(count).and_modify(|x| *x += 1).or_insert(1);
            }
        }

        // Incrementally make the largest frequencies "good" by removing all but 1
        loop {
            let max_key = if let Some(max_key) = freq_dist.keys().last() {
                *max_key
            } else {
                break;
            };
            let (count, multiplicity) = freq_dist.remove_entry(&max_key).unwrap();
            let excess = multiplicity - 1;
            freq_dist.remove(&count);
            if excess >= 1 {
                result += excess;
                freq_dist
                    .entry(count - 1)
                    .and_modify(|x| *x += excess)
                    .or_insert(excess);
            }
        }

        result
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("aab", 0)]
    #[case("aaabbbcc", 2)]
    #[case("ceabaacb", 2)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::min_deletions(s);
        assert_eq!(actual, expected);
    }
}
