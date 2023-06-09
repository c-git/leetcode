use std::{mem::swap, ops::Add};

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in 0..n {
            let val = nums[i];
            if val.saturating_sub(1) != i as i32 {
                Self::swap_sort(&mut nums, val);
            }
        }

        for (i, val) in nums.into_iter().enumerate() {
            if i as i32 != val.saturating_sub(1) {
                return i.add(1) as i32;
            }
        }

        // None missing must be n
        (n + 1) as i32
    }

    /// Take the curr_value and set the matching position if it is
    /// a positive integer within the size of the array and repeating this on the value
    /// at the location that it is swapped with
    fn swap_sort(nums: &mut [i32], mut curr_value: i32) {
        let n = nums.len() as i32;
        while 0 < curr_value
            && curr_value <= n
            && nums[curr_value.saturating_sub(1) as usize] != curr_value
        {
            // LI:
            //  - curr_value-1 is a valid index into nums because it is 0 < curr_value <= n implies 0 <= curr_value-1 < n
            //  - curr_value has somewhere else in the array it belongs
            let target_index = curr_value.saturating_sub(1) as usize;
            swap(&mut curr_value, &mut nums[target_index]);
        }
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,0],3)]
    #[case(vec![3,4,-1,1],2)]
    #[case(vec![7,8,9,11,12],1)]
    #[case(vec![1,0,0],2)]
    #[case(vec![1],2)]
    #[case(vec![2,1],3)]
    fn case(#[case] input: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::first_missing_positive(input);
        assert_eq!(actual, expected);
    }
}
