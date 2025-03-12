//! Solution for https://leetcode.com/problems/search-in-rotated-sorted-array-ii
//! 81. Search in Rotated Sorted Array II

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let n = nums.len();
        // Special case out short lists
        if n < 5 {
            return nums.iter().any(|x| x == &target);
        }

        // Find partition point
        let mut left = 0;
        let mut right = n - 1;
        while left < right && nums[left] >= nums[right] {
            while left < right && nums[left] == nums[right] {
                if nums[left] <= nums[left + 1] {
                    // Keep moving across the left list
                    left += 1;
                } else {
                    // We've found the pivot because the next cell is less than
                    // `nums[left]`
                    right = left;
                }
                continue;
            }

            let mid = (right + left) / 2;
            if nums[left] < nums[mid] {
                left = mid;
            } else {
                right = mid;
            }
        }

        if left == right {
            // Found a pivot search the correct side (pivot point is at end of left list)
            if target >= nums[0] {
                nums[..=left].binary_search(&target).is_ok()
            } else if left < n {
                nums[left + 1..].binary_search(&target).is_ok()
            } else {
                // Left list goes all the way to the end if it were included we would have gone on the first branch
                false
            }
        } else {
            // No pivot found do normal binary search
            nums.binary_search(&target).is_ok()
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
    #[case(vec![2,5,6,0,0,1,2], 0, true)]
    #[case(vec![2,5,6,0,0,1,2], 3, false)]
    #[case(vec![4,4,4,4,0,4,4], 0, true)]
    #[case(vec![4,4,4,4,0,2,3], 0, true)]
    #[case(vec![4,4,4,4], 0, false)]
    #[case(vec![4,4,4,4], 4, true)]
    #[case(vec![4,4,0], 0, true)]
    #[case(vec![2,2,2,3,2,2,2], 3, true)]
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: bool) {
        let actual = Solution::search(nums, target);
        assert_eq!(actual, expected);
    }
}
