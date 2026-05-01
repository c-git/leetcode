//! Solution for https://leetcode.com/problems/rotate-function
//! 396. Rotate Function

impl Solution {
    /// I was almost there but ran out of time and looked it up using https://www.youtube.com/watch?v=dsToMLJIXxQ
    /// I was missing that all increase except one (not some exactly 1)
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut nums_sum = 0;
        for (i, num) in nums.iter().copied().enumerate() {
            nums_sum += num;
            result += num * i as i32;
        }
        let mut current = result;
        let n = nums.len() as i32;
        for num in nums.iter().rev() {
            // Increase all by 1 except "last" which reduces total by length of
            // list * it's value which removes it 1 more time than it was
            // included but which is the right amount because we just added it
            // when we added the sum
            current = current + nums_sum - num * n;
            result = result.max(current);
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
    #[case(vec![4,3,2,6], 26)]
    #[case(vec![100], 0)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_rotate_function(nums);
        assert_eq!(actual, expected);
    }
}
