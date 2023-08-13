//! Solution for https://leetcode.com/problems/check-if-there-is-a-valid-partition-for-the-array
//! 2369. Check if There is a Valid Partition For The Array

impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let mut memo = vec![None; nums.len() + 1];
        Self::valid_partition_(&nums, &mut memo)
    }

    fn valid_partition_(nums: &[i32], memo: &mut Vec<Option<bool>>) -> bool {
        if let Some(value) = memo[nums.len()] {
            return value;
        }
        let result = match nums.len() {
            0 => true,  // Empty this is valid
            1 => false, // 1 element is never valid
            2 => nums[0] == nums[1],
            _ => {
                if nums[0] == nums[1] && Self::valid_partition_(&nums[2..], memo) {
                    true
                } else {
                    let three_equal = nums[0] == nums[1] && nums[1] == nums[2];
                    let diff_one = nums[1] - nums[0] == 1 && nums[2] - nums[1] == 1;
                    (three_equal || diff_one) && Self::valid_partition_(&nums[3..], memo)
                }
            }
        };
        memo[nums.len()] = Some(result);
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
    #[case(vec![4,4,4,5,6], true)]
    #[case(vec![1,1,1,2], false)]
    #[case(vec![1,1], true)]
    #[case(vec![1,1,1], true)]
    #[case(vec![1,2], false)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::valid_partition(nums);
        assert_eq!(actual, expected);
    }
}
