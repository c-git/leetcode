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
                    left += 1;
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

        // We should only be a few cases coming out of the loop
        // 1 - There existed a pivot and we found it, we are on the right end of the
        //      left list
        //      (left == right && (nums[0] != nums[left] || nums[left] != nums[n-1])
        // 2 - There was no pivot because all values were the same
        //      (left == right && nums[0] == nums[left] && nums[n-1] == nums[0])
        // 3 - There was no pivot so we never entered the loop
        //      (left != right && left == 0 && right == n - 1)
        if left == right {
            // We found a pivot let's see if it's real
            if nums[0] == nums[left] && nums[left] == nums[n - 1] {
                // We didn't actually find a pivot all values are equal
                debug_assert!(nums.iter().all(|&x| x == nums[0]));
                nums[0] == target
            } else {
                // We found a real pivot search the correct side
                if target >= nums[0] {
                    nums[..left].binary_search(&target).is_ok()
                } else {
                    nums[left..].binary_search(&target).is_ok()
                }
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
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: bool) {
        let actual = Solution::search(nums, target);
        assert_eq!(actual, expected);
    }
}
