//! Solution for https://leetcode.com/problems/total-characters-in-string-after-transformations-i
//! 3335. Total Characters in String After Transformations I

impl Solution {
    /// Based on editorial
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut char_count = [0; 26];
        for ch in s.chars() {
            char_count[(ch as u8 - b'a') as usize] += 1;
        }
        let mut alt_count = [0; 26]; // To avoid reallocation
        for _ in 0..t {
            alt_count[0] = char_count[25];
            alt_count[1] = (char_count[25] + char_count[0]) % MOD;
            alt_count[2..].clone_from_slice(&char_count[1..25]);
            std::mem::swap(&mut char_count, &mut alt_count);
        }

        char_count
            .into_iter()
            .fold(0, |acc, count| (acc + count) % MOD)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("abcyy", 2, 7)]
    #[case("azbk", 1, 5)]
    fn case(#[case] s: String, #[case] t: i32, #[case] expected: i32) {
        let actual = Solution::length_after_transformations(s, t);
        assert_eq!(actual, expected);
    }
}
