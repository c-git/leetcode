//! Solution for https://leetcode.com/problems/sort-an-array
//! 912. Sort an Array

impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        Self::merge_sort(&mut nums);
        nums
    }

    fn merge_sort(nums: &mut [i32]) {
        // Merge sort O(n) space usage (not minimal as question asked for just to see)
        let n = nums.len();
        if n < 2 {
            return; // Do nothing this is "sorted" already as 1 or no number cannot be in the wrong order
        }
        let mid = n / 2;
        let (left, right) = nums.split_at_mut(mid);
        Self::merge_sort(left);
        Self::merge_sort(right);
        let mut scratch = Vec::with_capacity(n);
        let mut ind_left = 0;
        let mut ind_right = 0;
        loop {
            match (left.get(ind_left), right.get(ind_right)) {
                (None, None) => break,
                (None, Some(&y)) => {
                    scratch.push(y);
                    ind_right += 1;
                }
                (Some(&x), None) => {
                    scratch.push(x);
                    ind_left += 1;
                }
                (Some(&x), Some(&y)) => {
                    if x < y {
                        scratch.push(x);
                        ind_left += 1;
                    } else {
                        scratch.push(y);
                        ind_right += 1;
                    }
                }
            }
        }
        for i in 0..n {
            if i < mid {
                left[i] = scratch[i];
            } else {
                right[i - mid] = scratch[i];
            }
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
    #[case(vec![5,2,3,1], vec![1,2,3,5])]
    #[case(vec![5,1,1,2,0,0], vec![0,0,1,1,2,5])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::sort_array(nums);
        assert_eq!(actual, expected);
    }
}
