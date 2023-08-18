//! Solution for https://leetcode.com/problems/jump-game-ii
//! 45. Jump Game II

impl Solution {
    pub fn jump(mut nums: Vec<i32>) -> i32 {
        // Using i32::MAX as impossible value because max array length is less than i32::MAX
        *nums.last_mut().unwrap() = 0;
        for i in (0..nums.len() - 1).rev() {
            let step_range = nums[i] as usize;
            let mut best = i32::MAX - 1;
            for &dst in nums.iter().skip(i + 1).take(step_range) {
                best = best.min(dst);
            }
            nums[i] = best + 1;
        }
        *nums.first().unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,3,1,1,4], 2)]
    #[case(vec![2,3,0,1,4], 2)]
    #[case(vec![1], 0)]
    #[case(vec![1,2,3], 2)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::jump(nums);
        assert_eq!(actual, expected);
    }
}
