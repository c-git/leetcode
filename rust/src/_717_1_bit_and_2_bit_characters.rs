//! Solution for https://leetcode.com/problems/1-bit-and-2-bit-characters
//! 717. 1-bit and 2-bit Characters

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut i = 0;
        while i < bits.len() - 1 {
            if bits[i] == 1 {
                // Skip the next bit as it's part the second character
                i += 1;
            }
            i += 1;
        }

        // If the next location we want to read is past the end then the last zero was
        // part of a second character (two bits)
        i < bits.len()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,0,0], true)]
    #[case(vec![1,1,1,0], false)]
    fn case(#[case] bits: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::is_one_bit_character(bits);
        assert_eq!(actual, expected);
    }
}
