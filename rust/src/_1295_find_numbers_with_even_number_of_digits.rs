//! Solution for https://leetcode.com/problems/find-numbers-with-even-number-of-digits
//! 1295. Find Numbers with Even Number of Digits

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for mut num in nums {
            let mut is_even = true;
            while num > 0 {
                is_even = !is_even;
                num /= 10;
            }
            if is_even {
                result += 1;
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
    #[case(vec![12,345,2,6,7896], 2)]
    #[case(vec![555,901,482,1771], 1)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::find_numbers(nums);
        assert_eq!(actual, expected);
    }
}
