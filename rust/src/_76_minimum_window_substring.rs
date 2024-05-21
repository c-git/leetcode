//! Solution for https://leetcode.com/problems/minimum-window-substring
//! 76. Minimum Window Substring

type Signature = [u16; 26 * 2];

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        // After using the hints on leetcode

        if t.is_empty() || s.len() < t.len() {
            return "".to_string();
        }

        let mut result: Option<[usize; 2]> = None;

        // Get signature of t
        let mut target_sig = [0; 26 * 2];
        for c in t.chars() {
            target_sig[Self::char_to_index(c)] += 1;
        }

        // Walk s to see if signature is matchable
        let s_array_indices: Vec<usize> = s.chars().map(Self::char_to_index).collect();
        let mut candidate_sig = [0; 26 * 2];
        let mut start = 0; // Inclusive
        for (end, next_index) in s_array_indices.iter().enumerate() {
            candidate_sig[*next_index] += 1;
            if Self::contains_sig(&candidate_sig, &target_sig) {
                // Remove from left until it is no longer contained
                loop {
                    debug_assert!(start <= end);
                    candidate_sig[s_array_indices[start]] -= 1;
                    start += 1;
                    if !Self::contains_sig(&candidate_sig, &target_sig) {
                        // No longer contains sig check if this one is the best seen
                        match &result {
                            Some([best_start, best_end])
                                if best_end - best_start < end - start + 2 => {} // No action needed already the best
                            _ => result = Some([start - 1, end + 1]), // Save current
                        }
                        break;
                    }
                }
            }
        }

        // Extract Result
        let [start, end] = result.unwrap_or([0, 0]);
        s[start..end].to_string()
    }

    #[inline]
    /// Converts a character into it's index in a signature
    ///
    /// # Panic
    /// Panics if char is not english uppercase or lowercase in debug, in prod it just assumes it's valid by if out of range will still panic
    fn char_to_index(c: char) -> usize {
        debug_assert!(
            c.is_alphabetic(),
            "only upper and lower case letters allowed"
        );
        let num = c as usize;
        if num < 97 {
            // Put upper case characters in first half
            debug_assert!((65..65 + 24).contains(&num));
            num - 65
        } else {
            // Put lower case characters in 2nd half
            debug_assert!((97..97 + 26).contains(&num));
            num - 97 + 26
        }
    }

    #[inline]
    fn contains_sig(candidate: &Signature, target: &Signature) -> bool {
        candidate.iter().zip(target).all(|(c, t)| c >= t)
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
    #[case("bcd", "a", "")]
    fn case(#[case] s: String, #[case] t: String, #[case] expected: String) {
        let actual = Solution::min_window(s, t);
        assert_eq!(actual, expected);
    }
}
