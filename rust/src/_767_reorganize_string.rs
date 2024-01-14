//! Solution for https://leetcode.com/problems/reorganize-string
//! 767. Reorganize String

use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        // Converted Beixuan's Python Solution to Rust
        let mut output = String::with_capacity(s.len());
        let mut c_occ: HashMap<char, usize> = HashMap::new();
        s.chars().for_each(|c| *c_occ.entry(c).or_default() += 1);
        let num_dist_c = c_occ.len();
        let max_occ = c_occ.values().max().unwrap();
        if *max_occ > (s.len() + 1) / 2 {
            return output;
        };
        let mut pairs = BinaryHeap::new();
        c_occ.into_iter().for_each(|(k, v)| pairs.push((v, k)));
        let mut pre = None;
        while let Some(curr) = pairs.pop() {
            output.push(curr.1);
            if let Some(val) = pre {
                pairs.push(val);
            }
            pre = if curr.0 > 1 {
                Some((curr.0 - 1, curr.1))
            } else {
                None
            };
        }
        output
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("aab", "aba")]
    #[case("aaab", "")]
    #[case("zhmyo", "zyomh")]
    #[case("ogccckcwmbmxtsbmozli", "cocgcickmlmsmtbwbxoz")]
    #[case("aabbcc", "abcabc")]
    fn case(#[case] s: String, #[case] expected: String) {
        let actual = Solution::reorganize_string(s);
        assert_eq!(actual.len(), expected.len(), "Length should be the same");
        check_no_repeats(actual);
    }

    fn check_no_repeats(actual: String) {
        let Some(mut last_char) = actual.chars().next() else {
            return;
        };
        for c in actual.chars().skip(1) {
            assert_ne!(c, last_char, "Duplicate character found in {actual}");
            last_char = c;
        }
    }
}
