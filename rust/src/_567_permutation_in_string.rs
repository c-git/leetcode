//! Solution for https://leetcode.com/problems/permutation-in-string
//! 567. Permutation in String

impl Solution {
    /// Based on https://www.youtube.com/watch?v=UbyhOgBN834
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();

        let mut s1_freq_count = vec![0; 26];
        let mut s2_freq_count = vec![0; 26];

        for i in 0..s1.len() {
            s1_freq_count[as_index(s1[i])] += 1;
            s2_freq_count[as_index(s2[i])] += 1;
        }

        let mut match_count: usize = (0..26)
            .map(|i| {
                if s1_freq_count[i] == s2_freq_count[i] {
                    1
                } else {
                    0
                }
            })
            .sum();
        if match_count == 26 {
            return true;
        }
        for (left_value, right_value) in s2.iter().copied().zip(s2.iter().skip(s1.len()).copied()) {
            increment_arr(
                right_value,
                &mut s2_freq_count,
                &s1_freq_count,
                &mut match_count,
                true,
            );
            increment_arr(
                left_value,
                &mut s2_freq_count,
                &s1_freq_count,
                &mut match_count,
                false,
            );
            if match_count == 26 {
                return true;
            }
        }
        false
    }
}

fn increment_arr(
    c: u8,
    s2_freq_count: &mut [usize],
    s1_freq_count: &[usize],
    match_count: &mut usize,
    should_increment: bool,
) {
    let index = as_index(c);
    let used_to_match = s1_freq_count[index] == s2_freq_count[index];
    if should_increment {
        s2_freq_count[index] += 1;
    } else {
        s2_freq_count[index] -= 1;
    }
    if used_to_match {
        *match_count -= 1;
    } else if s1_freq_count[index] == s2_freq_count[index] {
        *match_count += 1;
    }
}

fn as_index(c: u8) -> usize {
    (c - b'a') as usize
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("ab", "eidbaooo", true)]
    #[case("ab", "eidboaoo", false)]
    fn case(#[case] s1: String, #[case] s2: String, #[case] expected: bool) {
        let actual = Solution::check_inclusion(s1, s2);
        assert_eq!(actual, expected);
    }
}
