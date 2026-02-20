//! Solution for https://leetcode.com/problems/move-zeroes
//! 283. Move Zeroes

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut unused_idx = 0;

        // Move all non zero items to the front
        for unchecked_index in 0..nums.len() {
            if nums[unchecked_index] != 0 {
                nums[unused_idx] = nums[unchecked_index];
                unused_idx += 1;
            }
        }

        // Replace remaining positions with 0's
        for num in nums.iter_mut().skip(unused_idx) {
            *num = 0;
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![0,1,0,3,12], vec![1,3,12,0,0])]
    #[case(vec![0], vec![0])]
    fn case(#[case] mut nums: Vec<i32>, #[case] expected: Vec<i32>) {
        Solution::move_zeroes(&mut nums);
        let actual = nums;
        assert_eq!(actual, expected);
    }
}
