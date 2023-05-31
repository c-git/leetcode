impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 5 {
            return *nums.iter().min().expect("Constraint min length if 1");
        }

        // O(log n) - Find partition point (Observation: All values in the left sorted list are greater than the all values in the right side sort list)
        // left will be equal to right if there is a rotation and both on the end of the left side
        // If it is not rotated then they will not be equal and left == 0 and right == n-1
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
            // They are not equal so it is not rotated take first value
            debug_assert_eq!(left, 0);
            debug_assert_eq!(right, n - 1);
            nums[0]
        } else {
            // They are equal and thus must be on the end of the left side so take value at next index
            debug_assert_ne!(left, 0);
            nums[left + 1]
        }
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![3,4,5,1,2],1)]
    #[case(vec![4,5,6,7,0,1,2],0)]
    #[case(vec![11,13,15,17],11)]
    fn case(#[case] input: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::find_min(input);
        assert_eq!(actual, expected);
    }
}
