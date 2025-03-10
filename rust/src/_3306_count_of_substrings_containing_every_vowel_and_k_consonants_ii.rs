//! Solution for https://leetcode.com/problems/count-of-substrings-containing-every-vowel-and-k-consonants-ii
//! 3306. Count of Substrings Containing Every Vowel and K Consonants II

use std::ops::BitAnd;

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        let word = word.as_bytes();
        let mut vowel_present_bit_map = 0;
        let has_each_vowel = |bit_map| {
            debug_assert!(bit_map <= 31, "5 vowels with 1 bit each is 2^5-1");
            bit_map == 31
        };
        let mut vowel_count = [0i32; 5];
        let mut consonant_count = 0;
        // Creates a lower bound on the range we are looking at (Range is x such that left < x <= right)
        let mut left = 0;
        let mut removed_extra_vowels: Option<i64> = None;
        let mut result = 0;
        for (right, &char_at_right_of_range) in word.iter().enumerate() {
            if let Some(vowel_idx) = Self::vowel_to_idx(char_at_right_of_range) {
                vowel_increment(&mut vowel_present_bit_map, &mut vowel_count, vowel_idx);
            } else {
                consonant_count += 1;
            }
            while consonant_count > k {
                if let Some(vowel_idx) = Self::vowel_to_idx(word[left]) {
                    vowel_decrement(&mut vowel_present_bit_map, &mut vowel_count, vowel_idx);
                } else {
                    consonant_count -= 1;
                }
                left += 1;
                removed_extra_vowels = None; // The old ones are out of range now
            }
            if consonant_count == k && has_each_vowel(vowel_present_bit_map) {
                result += 1;
                let extras_from_removed_vowels = removed_extra_vowels.get_or_insert(0);
                while left < right {
                    if let Some(vowel_idx) = Self::vowel_to_idx(word[left]) {
                        vowel_decrement(&mut vowel_present_bit_map, &mut vowel_count, vowel_idx);
                        left += 1;
                        if has_each_vowel(vowel_present_bit_map) {
                            // Found unneeded vowel remove and make a record
                            *extras_from_removed_vowels += 1;
                        } else {
                            // Went too far... Undo
                            vowel_increment(
                                &mut vowel_present_bit_map,
                                &mut vowel_count,
                                vowel_idx,
                            );
                            left -= 1;
                            debug_assert!(has_each_vowel(vowel_present_bit_map));
                            break;
                        }
                    } else {
                        break;
                    }
                }
                result += *extras_from_removed_vowels;
            }
        }
        result
    }

    fn vowel_to_idx(vowel: u8) -> Option<usize> {
        match vowel {
            b'a' => Some(0),
            b'e' => Some(1),
            b'i' => Some(2),
            b'o' => Some(3),
            b'u' => Some(4),
            _ => None,
        }
    }
}

fn vowel_decrement(vowel_present_bit_map: &mut i32, vowel_count: &mut [i32; 5], vowel_idx: usize) {
    vowel_count[vowel_idx] = vowel_count[vowel_idx].saturating_sub(1);
    let bit_mask = 1 << vowel_idx;
    if vowel_count[vowel_idx] == 0 && vowel_present_bit_map.bitand(bit_mask) > 0 {
        // Unset bit
        *vowel_present_bit_map ^= bit_mask;
    }
}

fn vowel_increment(vowel_present_bit_map: &mut i32, vowel_count: &mut [i32; 5], vowel_idx: usize) {
    vowel_count[vowel_idx] += 1;
    *vowel_present_bit_map |= 1 << vowel_idx;
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("aeioqq", 1, 0)]
    #[case("aeiou", 0, 1)]
    #[case("ieaouqqieaouqq", 1, 3)]
    #[case("iqeaouqi", 2, 3)]
    #[case("iqeaouqii", 2, 5)]
    fn case(#[case] word: String, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::count_of_substrings(word, k);
        assert_eq!(actual, expected);
    }
}
