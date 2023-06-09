use std::mem::swap;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in 0..n {
            if nums[i] != i as i32 {
                Self::swap_sort(&mut nums, i);
            }
        }

        for (i, val) in nums.into_iter().enumerate().skip(1) {
            if i as i32 != val {
                return i as i32;
            }
        }

        // None missing must be n
        n as i32
    }

    /// Take the value at nums[index] and set that position in the array to the value if it
    /// is a positive integer within the size of the array and repeating on the value
    /// at that location as appropriate
    fn swap_sort(nums: &mut [i32], index: usize) {
        debug_assert_ne!(nums[index], index as i32);

        let mut curr_value = nums[index];
        let n = nums.len() as i32;
        while 0 <= curr_value && curr_value < n && nums[curr_value as usize] != curr_value {
            // LI:
            //  - curr_value is a valid index into nums because 0 <= index < n
            //  - curr_value has somewhere else in the array it belongs
            let target_index = curr_value as usize;
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
    fn case(#[case] input: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::first_missing_positive(input);
        assert_eq!(actual, expected);
    }
}
