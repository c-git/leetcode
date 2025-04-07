//! Solution for https://leetcode.com/problems/find-all-anagrams-in-a-string
//! 438. Find All Anagrams in a String

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut result = vec![];
        if s.len() < p.len() {
            return result;
        }
        let s = s.as_bytes();
        let mut s_characteristic = [0; 26];
        let mut p_characteristic = [0; 26];

        // Record frequency of each char in p
        for c in p.as_bytes().iter() {
            p_characteristic[(c - b'a') as usize] += 1;
        }

        // Record frequency of each char in s
        for c in s.iter().take(p.len()) {
            s_characteristic[(c - b'a') as usize] += 1;
        }

        let mut match_count = s_characteristic
            .iter()
            .zip(p_characteristic.iter())
            .filter(|(x, y)| x == y)
            .count();
        if match_count == 26 {
            result.push(0);
        }

        // Count number of matches
        for (right_idx, right_val) in s.iter().enumerate().skip(p.len()) {
            let left_idx = right_idx - p.len();
            let left_char_idx = (s[left_idx] - b'a') as usize;
            let right_char_idx = (right_val - b'a') as usize;
            let is_left_match_before =
                s_characteristic[left_char_idx] == p_characteristic[left_char_idx];
            let is_right_match_before =
                s_characteristic[right_char_idx] == p_characteristic[right_char_idx];
            let is_left_same_char_as_right = left_char_idx == right_char_idx;
            s_characteristic[left_char_idx] -= 1;
            s_characteristic[right_char_idx] += 1;
            let is_left_match_after =
                s_characteristic[left_char_idx] == p_characteristic[left_char_idx];
            let is_right_match_after =
                s_characteristic[right_char_idx] == p_characteristic[right_char_idx];
            if !is_left_same_char_as_right {
                if is_left_match_before != is_left_match_after {
                    if is_left_match_after {
                        match_count += 1;
                    } else {
                        match_count -= 1;
                    }
                }
                if is_right_match_before != is_right_match_after {
                    if is_right_match_after {
                        match_count += 1;
                    } else {
                        match_count -= 1;
                    }
                }
            } else {
                // When equal the count cannot change
            }
            if match_count == 26 {
                result.push(left_idx as i32 + 1);
            }
        }
        result
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("cbaebabacd", "abc", vec![0,6])]
    #[case("abab", "ab", vec![0,1,2])]
    fn case(#[case] s: String, #[case] p: String, #[case] expected: Vec<i32>) {
        let actual = Solution::find_anagrams(s, p);
        assert_eq!(actual, expected);
    }
}
