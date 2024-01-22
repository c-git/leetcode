//! Solution for https://leetcode.com/problems/set-mismatch
//! 645. Set Mismatch

impl Solution {
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        let mut duplicated = None;
        let mut missing = None;
        let mut next_expected = Some(1);
        nums.sort_unstable();
        if Some(nums[0]) != next_expected {
            missing = next_expected;
            next_expected = None;
        } else {
            next_expected = Some(2);
        }
        for i in 1.. {
            if duplicated.is_some() && missing.is_some() {
                // Found both we can stop
                break;
            }
            if let Some(current_num) = nums.get(i) {
                if &nums[i - 1] == current_num {
                    debug_assert!(duplicated.is_none());
                    duplicated = Some(nums[i]);
                } else if let Some(next_val) = next_expected {
                    next_expected = if current_num != &next_val {
                        debug_assert!(missing.is_none());
                        missing = next_expected;
                        None
                    } else {
                        Some(next_val + 1)
                    };
                }
            } else {
                missing = next_expected;
            }
        }

        vec![
            duplicated.expect("Should exist by problem constraints"),
            missing.expect("Should exist by problem constraints"),
        ]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,2,4], vec![2,3])]
    #[case(vec![1,1], vec![1,2])]
    #[case(vec![1,2,2,3,4], vec![2,5])]
    #[case(vec![2,2,3,4], vec![2,1])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::find_error_nums(nums);
        assert_eq!(actual, expected);
    }
}
