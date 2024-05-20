//! Solution for https://leetcode.com/problems/sort-colors
//! 75. Sort Colors

#[allow(clippy::ptr_arg)]
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // Trying bubble sort to see if it will get TLE
        let mut unsorted = true;
        while unsorted {
            unsorted = false;
            for i in 0..nums.len() - 1 {
                if nums[i] > nums[i + 1] {
                    nums.swap(i, i + 1);
                    unsorted = true;
                }
            }
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,0,2,1,1,0])]
    #[case(vec![2,0,1])]
    fn case(#[case] mut nums: Vec<i32>) {
        Solution::sort_colors(&mut nums);
        validate(&nums);
    }

    /// Ensure we only see each number in contiguous blocks
    fn validate(nums: &[i32]) {
        let mut seen = HashSet::new();
        let Some(mut last_seen) = nums.first() else {
            return; // Empty is fine
        };
        seen.insert(last_seen);
        for num in nums.iter().skip(1) {
            if num != last_seen {
                assert!(
                    seen.insert(num),
                    "{num} seen twice no adjacent to each other"
                );
                last_seen = num;
            }
        }
    }
}
