//! Solution for https://leetcode.com/problems/house-robber
//! 198. House Robber

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        // Going 3 back should be enough because if we had to go 4 back then it would have been better to go 2 back already
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        if n == 2 {
            return nums[0].max(nums[1]);
        }

        let [mut best_back_three, mut best_back_two] = nums[0..=1] else {
            unreachable!("Length is 3 or more because of constraint that says it must be 1 or more and we already checked for 1 and 2");
        };
        let mut best_back_one = nums[2] + best_back_three;

        for current in nums.iter().skip(3) {
            let best_current = current + best_back_two.max(best_back_three);
            (best_back_one, best_back_two, best_back_three) =
                (best_current, best_back_one, best_back_two);
        }
        best_back_one.max(best_back_two)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,1],  4)]
    #[case(vec![2,7,9,3,1], 12)]
    #[case(vec![2,1,1,2],  4)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::rob(nums);
        assert_eq!(actual, expected);
    }
}
