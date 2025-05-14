//! Solution for https://leetcode.com/problems/total-characters-in-string-after-transformations-ii
//! 3337. Total Characters in String After Transformations II

impl Solution {
    pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut char_count = [0; 26];
        for ch in s.chars() {
            char_count[(ch as u8 - b'a') as usize] += 1;
        }
        for _ in 0..t {
            let mut alt_count = [0; 26];
            for (char_idx, shift_count) in nums
                .iter()
                .map(|&shift_count| shift_count as usize)
                .enumerate()
            {
                for shift in 1..=shift_count {
                    let new_idx = (char_idx + shift) % 26;
                    alt_count[new_idx] += char_count[char_idx];
                }
            }
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
    #[case("abcyy", 2, vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2], 7)]
    #[case("azbk", 1, vec![2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2], 8)]
    fn case(#[case] s: String, #[case] t: i32, #[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::length_after_transformations(s, t, nums);
        assert_eq!(actual, expected);
    }
}
