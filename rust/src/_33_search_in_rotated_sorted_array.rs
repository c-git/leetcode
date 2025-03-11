impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if let Some(result) = Self::_search(&nums, target) {
            result as i32
        } else {
            -1
        }
    }

    fn _search(nums: &[i32], target: i32) -> Option<usize> {
        let n = nums.len();
        // Special case out short lists
        if n < 5 {
            return nums
                .iter()
                .enumerate()
                .find_map(|(i, &x)| if x == target { Some(i) } else { None });
        }

        // O(log n) - Find partition point
        //
        // If there is no rotation then then both `left` and `right` will not change
        // from `left == 0` and `right n-1` But if there is a rotation then
        // after finding the partition point `left` will be equal to `right` and both
        // `left` and `right` will be on the right (high value) end of the left list,
        // which is at the partition point.
        //
        // Observation: All values in the left sorted list are greater than all the
        // values in the right side sort list.
        let mut left = 0;
        let mut right = n - 1;
        while left < right && nums[left] > nums[right] {
            let mid = (right + left) / 2;
            if nums[left] < nums[mid] {
                // mid is in the left side
                left = mid;
            } else {
                // mid is in the right side
                right = mid;
            }
        }
        if left != right {
            // They are not equal, which means that there is no rotation so just use built
            // in binary search on full array
            debug_assert_eq!(left, 0);
            debug_assert_eq!(right, n - 1);
            return match nums.binary_search(&target) {
                Ok(pos) => Some(pos),
                Err(_) => None,
            };
        } else {
            // They are equal and thus must be on the right end of the left list
            debug_assert_ne!(left, 0);
        }

        // O(log n) - Find value
        debug_assert!(left < n - 1);
        if target < nums[0] {
            // If target exits it must be in the right list
            let start_of_right_list = left + 1;
            nums[start_of_right_list..]
                .binary_search(&target)
                .ok()
                .map(|pos| pos + start_of_right_list)
        } else {
            // If target exits it must be in the left list
            let end_of_left_list = left;
            nums[..=end_of_left_list].binary_search(&target).ok()
        }
    }
}

pub struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![4,5,6,7,0,1,2],0,4)]
    #[case(vec![4,5,6,7,0,1,2],3,-1)]
    #[case(vec![1],0,-1)]
    #[case(vec![0,1,2,4,5,6,7],2,2)]
    #[case(vec![1,2,4,5,6,7,0],2,1)]
    #[case(vec![1,2,4,5,6,7,0],0,6)]
    #[case(vec![4,5,6,7,0,1,2],2,6)]
    #[case(vec![4,5,6,7,0,1,2],1,5)]
    #[case(vec![4,5,6,7,0,1,2],7,3)]
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: i32) {
        let actual = Solution::search(nums, target);
        assert_eq!(actual, expected);
    }
}
