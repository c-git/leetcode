//! Solution for https://leetcode.com/problems/3sum
//! 15. 3Sum

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        nums.sort_unstable();
        for (first_idx, first_val) in nums.iter().copied().enumerate() {
            if first_idx > 0 && first_val == nums[first_idx - 1] {
                // Skip duplicated values
                continue;
            }
            let mut left = first_idx + 1;
            let mut right = nums.len() - 1;
            while left < right {
                if left > first_idx + 1 && nums[left] == nums[left - 1] {
                    // Skip duplicates
                    left += 1;
                    continue;
                }
                if right < nums.len() - 1 && nums[right] == nums[right + 1] {
                    // Skip duplicates
                    right -= 1;
                    continue;
                }
                let curr = first_val + nums[left] + nums[right];
                if curr == 0 {
                    result.push(vec![first_val, nums[left], nums[right]]);
                }
                if curr > 0 {
                    right -= 1;
                } else {
                    left += 1;
                }
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
    #[case(vec![-1,0,1,2,-1,-4], vec![vec![-1,-1,2],vec![-1,0,1]])]
    #[case(vec![0,1,1], vec![])]
    #[case(vec![0,0,0], vec![vec![0,0,0]])]
    #[case(vec![0,0,0,0], vec![vec![0,0,0]])]
    fn case(#[case] nums: Vec<i32>, #[case] mut expected: Vec<Vec<i32>>) {
        let mut actual = Solution::three_sum(nums);
        actual.iter_mut().for_each(|x| x.sort_unstable());
        actual.sort_unstable();
        expected.iter_mut().for_each(|x| x.sort_unstable());
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }
}
