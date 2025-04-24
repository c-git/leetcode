//! Solution for https://leetcode.com/problems/median-of-two-sorted-arrays
//! 4. Median of Two Sorted Arrays

impl Solution {
    /// Based on https://www.youtube.com/watch?v=q6IEA26hvXc
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        let total_len = nums1.len() + nums2.len();
        let half_len = total_len / 2;
        // Ensure nums1 is shorter as we are going to binary search on that one
        if nums1.len() > nums2.len() {
            std::mem::swap(&mut nums1, &mut nums2);
        }
        let mut left = 0;
        let mut right = nums1.len() as i32 - 1;
        loop {
            let nums1_mid = (left + right).div_euclid(2);
            let nums2_mid = half_len as i32 - nums1_mid - 2;

            let nums1_left = if nums1_mid >= 0 {
                nums1[nums1_mid as usize]
            } else {
                i32::MIN
            };

            let nums1_right = if nums1_mid + 1 < nums1.len() as i32 {
                nums1[(nums1_mid + 1) as usize]
            } else {
                i32::MAX
            };

            let nums2_left = if nums2_mid >= 0 {
                nums2[nums2_mid as usize]
            } else {
                i32::MIN
            };

            let nums2_right = if nums2_mid + 1 < nums2.len() as i32 {
                nums2[(nums2_mid + 1) as usize]
            } else {
                i32::MAX
            };

            if nums1_left <= nums2_right && nums2_left <= nums1_right {
                // Partition is not correct, return median
                if total_len % 2 == 0 {
                    // Even
                    return (nums1_left.max(nums2_left) + nums1_right.min(nums2_right)) as f64
                        / 2.0;
                } else {
                    // Odd
                    return nums1_right.min(nums2_right).into();
                }
            } else if nums1_left > nums2_right {
                right = nums1_mid - 1
            } else if nums2_left > nums1_right {
                left = nums1_mid + 1
            } else {
                unreachable!("One of the two above should have triggered")
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
    #[case(vec![1,3], vec![2], 2.0)]
    #[case(vec![1,2], vec![3,4], 2.5)]
    #[case(vec![1,5,7], vec![2,4,8], 4.5)]
    fn case(#[case] nums1: Vec<i32>, #[case] nums2: Vec<i32>, #[case] expected: f64) {
        let actual = Solution::find_median_sorted_arrays(nums1, nums2);
        assert!((actual - expected).abs() < 1e-5, "Assertion failed: actual {actual:.5} but expected {expected:.5}. Diff is more than 1e-5.");
    }
}
