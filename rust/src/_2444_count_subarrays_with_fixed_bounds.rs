//! Solution for https://leetcode.com/problems/count-subarrays-with-fixed-bounds
//! 2444. Count Subarrays With Fixed Bounds

impl Solution {
    /// Based on https://www.youtube.com/watch?v=Bk-HxzaooqM
    /// I wasn't fully sold by his explanation but the code matches what I
    /// eventually understood.
    ///
    /// My understanding is that you want
    /// - Only count subarrays when you have both exactly the correct min and
    ///   max in the current partition being considered
    /// - Count the number of subarrays you can make until you can't go back any
    ///   further because there is an out of bounds value using the smaller of
    ///   the min and max as the point from where we can take each subset
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut result = 0;
        // First valid position we can use
        let Some(mut partition_start) = nums.iter().enumerate().find_map(|(i, &num)| {
            if min_k <= num && num <= max_k {
                Some(i)
            } else {
                None
            }
        }) else {
            return result;
        };
        let mut closest_min = None;
        let mut closest_max = None;
        for (i, num) in nums.into_iter().enumerate().skip(partition_start) {
            if num < min_k || num > max_k {
                closest_min = None;
                closest_max = None;
                partition_start = i + 1; // Let's try for the next index
                continue;
            }
            if num == min_k {
                closest_min = Some(i);
            }
            if num == max_k {
                closest_max = Some(i);
            }
            if let (Some(min), Some(max)) = (closest_min, closest_max) {
                // We have the min and max in range so calculate how many at the start can be
                // left out plus one for none included
                result += (std::cmp::min(min, max) - partition_start + 1) as i64
            }
        }
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
    #[case(vec![1,3,5,2,7,5], 1, 5, 2)]
    #[case(vec![1,1,1,1], 1, 1, 10)]
    fn case(#[case] nums: Vec<i32>, #[case] min_k: i32, #[case] max_k: i32, #[case] expected: i64) {
        let actual = Solution::count_subarrays(nums, min_k, max_k);
        assert_eq!(actual, expected);
    }
}
