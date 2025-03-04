//! Solution for https://leetcode.com/problems/4sum
//! 18. 4Sum

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return Default::default();
        }
        let mut result = vec![];
        nums.sort_unstable();
        let mut scratch = vec![0; 4];
        Self::n_sum(&mut scratch, &nums, 0, target, &mut result);
        result
    }

    fn n_sum(
        scratch: &mut [i32],
        nums: &[i32],
        pos: usize,
        target: i32,
        result: &mut Vec<Vec<i32>>,
    ) {
        // Base case looking for last value call function to binary search
        if pos == scratch.len() - 1 {
            Self::check_for_last(scratch, nums, target, result);
            return;
        }

        // General case - add next
        for i in 0..nums.len() + 1 + pos - scratch.len() {
            if i > 0 && nums[i - 1] == nums[i] {
                // Already tried this value in this position
                continue;
            }

            scratch[pos] = nums[i];
            let Some(new_target) = target.checked_sub(nums[i]) else {
                // Target has gone out of bounds, don't think we can still make the target
                return;
            };
            Self::n_sum(scratch, &nums[i + 1..], pos + 1, new_target, result);
        }
    }

    fn check_for_last(scratch: &mut [i32], nums: &[i32], target: i32, result: &mut Vec<Vec<i32>>) {
        if nums.binary_search(&target).is_ok() {
            *scratch.last_mut().unwrap() = target;
            result.push(scratch.to_vec());
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
    #[case(vec![1,0,-1,0,-2,2], 0, vec![vec![-2,-1,1,2],vec![-2,0,0,2],vec![-1,0,0,1]])]
    #[case(vec![2,2,2,2,2], 8, vec![vec![2,2,2,2]])]
    #[case(vec![0], 0, vec![])]
    #[case(vec![1_000_000_000,1_000_000_000,1_000_000_000,1_000_000_000], -294_967_296, vec![])]
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] mut expected: Vec<Vec<i32>>) {
        let mut actual = Solution::four_sum(nums, target);
        actual.iter_mut().for_each(|x| x.sort_unstable());
        actual.sort_unstable();
        expected.iter_mut().for_each(|x| x.sort_unstable());
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }
}
