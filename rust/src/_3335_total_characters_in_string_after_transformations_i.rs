//! Solution for https://leetcode.com/problems/total-characters-in-string-after-transformations-i
//! 3335. Total Characters in String After Transformations I

impl Solution {
    /// From editorial
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut cnt = [0; 26];
        for ch in s.chars() {
            cnt[(ch as u8 - b'a') as usize] += 1;
        }
        for _ in 0..t {
            let mut nxt = [0; 26];
            nxt[0] = cnt[25];
            nxt[1] = (cnt[25] + cnt[0]) % MOD;
            for i in 2..26 {
                nxt[i] = cnt[i - 1];
            }
            cnt = nxt;
        }
        let mut ans = 0;
        for &num in cnt.iter() {
            ans = (ans + num) % MOD;
        }
        ans
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
