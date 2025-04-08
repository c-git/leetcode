//! Solution for https://leetcode.com/problems/minimum-window-substring
//! 76. Minimum Window Substring

use std::collections::{hash_map::Entry, HashMap};

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        Self::min_window_(s, t).unwrap_or_default()
    }

    fn min_window_(s: String, t: String) -> Option<String> {
        if s.len() < t.len() {
            return None;
        }
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut result_start_idx: Option<usize> = None; // Stores the best start index found
        let mut result_shortest_len = usize::MAX; // Stores the best length found
        let mut t_characteristic: HashMap<u8, i32> = HashMap::new();
        for c in t {
            *t_characteristic.entry(*c).or_default() += 1;
        }

        // Start same as t and subtract until all reach or pass zero
        let mut s_characteristic: HashMap<u8, i32> = t_characteristic.clone();
        let mut incomplete_char_freq = s_characteristic.len(); // Start with all missing
        let mut left_idx = 0;

        for (right_idx, right_val) in s.iter().enumerate() {
            if let Entry::Occupied(mut occupied_entry) = s_characteristic.entry(*right_val) {
                // Found a character we are interested in
                let val = occupied_entry.get_mut();
                *val -= 1; // Record that we need one less
                if *val == 0 {
                    // Reached 0 mark as no longer needed
                    incomplete_char_freq -= 1;
                }
                while incomplete_char_freq == 0 {
                    // Keep shortening as long as we still have all the required chars
                    let len = right_idx - left_idx + 1;
                    if len < result_shortest_len {
                        result_shortest_len = len;
                        result_start_idx = Some(left_idx);
                    }

                    if let Entry::Occupied(mut occupied_entry) = s_characteristic.entry(s[left_idx])
                    {
                        // Found a character that we need to add back
                        let val = occupied_entry.get_mut();
                        *val += 1;
                        if *val == 1 {
                            incomplete_char_freq += 1;
                        }
                    }

                    // Move left end forward
                    left_idx += 1;
                }
            }
        }

        // Convert back into a string
        let start_idx = result_start_idx?;
        let end_idx = start_idx + result_shortest_len;
        Some(
            std::str::from_utf8(&s[start_idx..end_idx])
                .unwrap()
                .to_string(),
        )
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("ADOBECODEBANC", "ABC", "BANC")]
    #[case("a", "a", "a")]
    #[case("a", "aa", "")]
    fn case(#[case] s: String, #[case] t: String, #[case] expected: String) {
        let actual = Solution::min_window(s, t);
        assert_eq!(actual, expected);
    }
}
