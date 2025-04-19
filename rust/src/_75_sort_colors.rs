//! Solution for https://leetcode.com/problems/sort-colors
//! 75. Sort Colors

impl Solution {
    pub fn sort_colors(nums: &mut [i32]) {
        // Use radix sort for linear runtime
        let mut freq_counts = [0; 3];
        for &num in nums.iter() {
            freq_counts[num as usize] += 1;
        }
        let mut done = 0;
        for color in 0..freq_counts.len() as i32 {
            for num in nums.iter_mut().skip(done).take(freq_counts[color as usize]) {
                *num = color;
            }
            done += freq_counts[color as usize];
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
