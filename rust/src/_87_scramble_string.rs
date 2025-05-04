//! Solution for https://leetcode.com/problems/scramble-string
//! 87. Scramble String

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        Self::is_scramble_(s1.as_bytes(), s2.as_bytes())
    }

    pub fn is_scramble_(s1: &[u8], s2: &[u8]) -> bool {
        debug_assert_eq!(s1.len(), s2.len());
        if s1.is_empty() {
            return true;
        }
        let n = s1.len();

        // Pick off ends if possible
        if s1[0] == s2[0] {
            return Self::is_scramble_(&s1[1..], &s2[1..]);
        }
        if s1[n - 1] == s2[n - 1] {
            return Self::is_scramble_(&s1[..n - 1], &s2[..n - 1]);
        }
        if s1[0] == s2[n - 1] {
            return Self::is_scramble_(&s1[1..], &s2[..n - 1]);
        }
        if s1[n - 1] == s2[0] {
            return Self::is_scramble_(&s1[..n - 1], &s2[1..]);
        }

        // Take span up to where characters are equal and split it out
        let mut seen_s1 = [0; 26];
        let mut seen_s2 = [0; 26];
        for i in 0..n - 1 {
            seen_s1[(s1[i] - b'a') as usize] += 1;
            seen_s2[(s2[i] - b'a') as usize] += 1;
            if seen_s1 == seen_s2 {
                return Self::is_scramble_(&s1[..=i], &s2[..=i])
                    && Self::is_scramble_(&s1[i + 1..], &s2[i + 1..]);
            }
        }

        // We only get here if we can't find the split which only happens if the split is not possible
        false
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("great", "rgeat", true)]
    #[case("abcde", "caebd", false)]
    #[case("a", "a", true)]
    #[case("abcde", "cabed", true)]
    fn case(#[case] s1: String, #[case] s2: String, #[case] expected: bool) {
        let actual = Solution::is_scramble(s1, s2);
        assert_eq!(actual, expected);
    }
}
