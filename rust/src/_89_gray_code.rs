//! Solution for https://leetcode.com/problems/gray-code
//! 89. Gray Code

impl Solution {
    /// Based on https://www.youtube.com/watch?v=xG8qJ_U0XPg
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut result = vec![0];
        for i in 1..=n as usize {
            let flip_bit = 1 << (i - 1);
            let old_len = result.len();
            for copy_idx in (0..old_len).rev() {
                result.push(result[copy_idx] ^ flip_bit);
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
    #[case(2, vec![0,1,3,2])]
    #[case(1, vec![0,1])]
    fn case(#[case] n: i32, #[case] expected: Vec<i32>) {
        let actual = Solution::gray_code(n);
        assert_eq!(actual, expected);
    }
}
