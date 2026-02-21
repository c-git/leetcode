//! Solution for https://leetcode.com/problems/median-of-two-sorted-arrays
//! 4. Median of Two Sorted Arrays

impl Solution {
    /// I think I finally grokked it after watching Neetcode's video again
    /// https://www.youtube.com/watch?v=q6IEA26hvXc (Not even sure how many times I've watched it now)
    ///
    /// We want to partition the full data which is spread across the two arrays
    /// into equal (or almost equal) halves. We do this by assuming that we take
    /// half of one list and the rest from the other. Then based on if the
    /// halves match we adjust until we get the right amount from that list
    /// then the rest must come from the other (may end up being all or none
    /// from the list we start with)
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total_count = (nums1.len() + nums2.len()) as i32;
        let half_count = total_count / 2;

        // Set the bounds for the search space to partition nums1
        let mut search_lower_limit = 0;
        let mut search_upper_limit = (nums1.len() - 1) as i32;
        while !partitioned_correctly(
            &nums1,
            &nums2,
            half_count,
            search_lower_limit,
            search_upper_limit,
        ) {
            // LI:  - The partitioning is not correct yet, by loop entry condition
            //      - If there are numbers within nums1 that are in the half of smaller
            //        numbers the right index of those numbers is between
            //        `search_lower_limit` and `search_upper_limit`
            let (
                num1_left_end,
                _num1_right_start,
                _num2_left_end,
                num2_right_start,
                num1_left_end_candidate_index,
            ) = partition_endings(
                &nums1,
                &nums2,
                half_count,
                search_lower_limit,
                search_upper_limit,
            );
            if num1_left_end > num2_right_start {
                search_upper_limit = num1_left_end_candidate_index - 1;
            } else {
                search_lower_limit = num1_left_end_candidate_index + 1;
            }
        }

        let (
            num1_left_end,
            num1_right_start,
            num2_left_end,
            num2_right_start,
            _num1_left_end_candidate_index,
        ) = partition_endings(
            &nums1,
            &nums2,
            half_count,
            search_lower_limit,
            search_upper_limit,
        );
        if total_count % 2 == 0 {
            (num1_left_end.max(num2_left_end) + num1_right_start.min(num2_right_start)) as f64 / 2.0
        } else {
            num1_right_start.min(num2_right_start) as f64
        }
    }
}

fn partitioned_correctly(
    nums1: &[i32],
    nums2: &[i32],
    half_count: i32,
    search_lower_limit: i32,
    search_upper_limit: i32,
) -> bool {
    let (
        num1_left_end,
        num1_right_start,
        num2_left_end,
        num2_right_start,
        _num1_left_end_candidate_index,
    ) = partition_endings(
        nums1,
        nums2,
        half_count,
        search_lower_limit,
        search_upper_limit,
    );

    num1_left_end <= num2_right_start && num2_left_end <= num1_right_start
}

fn partition_endings(
    nums1: &[i32],
    nums2: &[i32],
    half_count: i32,
    search_lower_limit: i32,
    search_upper_limit: i32,
) -> (i32, i32, i32, i32, i32) {
    let num1_left_end_candidate_index = (search_lower_limit + search_upper_limit).div_euclid(2);

    let num2_left_end_candidate_index = half_count - 2 - num1_left_end_candidate_index;

    let num1_left_end = if num1_left_end_candidate_index >= 0 {
        nums1[num1_left_end_candidate_index as usize]
    } else {
        i32::MIN
    };

    let num1_right_start = *nums1
        .get((num1_left_end_candidate_index + 1) as usize)
        .unwrap_or(&i32::MAX);

    let num2_left_end = if num2_left_end_candidate_index >= 0 {
        nums2[num2_left_end_candidate_index as usize]
    } else {
        i32::MIN
    };

    let num2_right_start = *nums2
        .get((num2_left_end_candidate_index + 1) as usize)
        .unwrap_or(&i32::MAX);

    (
        num1_left_end,
        num1_right_start,
        num2_left_end,
        num2_right_start,
        num1_left_end_candidate_index,
    )
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3], vec![2], 2.00000)]
    #[case(vec![1,2], vec![3,4], 2.50000)]
    #[case(vec![1,5,7], vec![2,4,8], 4.5)]
    fn case(#[case] nums1: Vec<i32>, #[case] nums2: Vec<i32>, #[case] expected: f64) {
        let actual = Solution::find_median_sorted_arrays(nums1, nums2);
        assert!(
            (actual - expected).abs() < 1e-5,
            "Assertion failed: actual {actual:.5} but expected {expected:.5}. Diff is more than 1e-5."
        );
    }
}
