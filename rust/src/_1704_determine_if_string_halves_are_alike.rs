//! Solution for https://leetcode.com/problems/determine-if-string-halves-are-alike
//! 1704. Determine if String Halves Are Alike

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let mut vowel_count = 0;
        let vowels = [b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U'];
        let chars = s.as_bytes();
        let half_len = chars.len() / 2;
        for (i, c) in chars.iter().enumerate() {
            if vowels.contains(c) {
                if i < half_len {
                    vowel_count += 1;
                } else {
                    vowel_count -= 1;
                }
            }
        }
        vowel_count == 0
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("book", true)]
    #[case("textbook", false)]
    #[case("AbCdEfGh", true)]
    fn case(#[case] s: String, #[case] expected: bool) {
        let actual = Solution::halves_are_alike(s);
        assert_eq!(actual, expected);
    }
}
