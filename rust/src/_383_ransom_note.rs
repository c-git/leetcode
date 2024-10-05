//! Solution for https://leetcode.com/problems/ransom-note
//! 383. Ransom Note

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        // Assumption: Using constraint from problem: `ransomNote` and `magazine` consist of lowercase English letters.

        let mut available_chars = vec![0usize; 26];

        for c in magazine.as_bytes() {
            let idx = (c - b'a') as usize;
            available_chars[idx] += 1;
        }

        for c in ransom_note.as_bytes() {
            let idx = (c - b'a') as usize;
            if available_chars[idx] == 0 {
                return false;
            } else {
                available_chars[idx] -= 1;
            }
        }
        true
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("a", "b", false)]
    #[case("aa", "ab", false)]
    #[case("aa", "aab", true)]
    fn case(#[case] ransom_note: String, #[case] magazine: String, #[case] expected: bool) {
        let actual = Solution::can_construct(ransom_note, magazine);
        assert_eq!(actual, expected);
    }
}
