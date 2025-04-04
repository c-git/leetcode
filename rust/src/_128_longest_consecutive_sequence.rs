//! Solution for https://leetcode.com/problems/longest-consecutive-sequence
//! 128. Longest Consecutive Sequence

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // Intuition: Build a HashSet with all the values then walk over the data again
        // and check what the longest sequence you can find is, clearing the values as
        // you walk so that you don't walk it more than once
        let mut result = 0;

        // Build HashSet
        let mut hash_set = std::collections::HashSet::with_capacity(nums.len());
        for num in nums.iter().copied() {
            hash_set.insert(num);
        }

        // Check sequence lengths in HashSet and empty set to avoid repeated walks
        for num in nums {
            let is_already_seen = !hash_set.remove(&num);
            if is_already_seen {
                continue;
            }

            let mut seq_len = 1;

            // Count backwards
            let mut next_value = num - 1;
            while hash_set.remove(&next_value) {
                seq_len += 1;
                next_value -= 1;
            }

            // Count forward
            next_value = num + 1;
            while hash_set.remove(&next_value) {
                seq_len += 1;
                next_value += 1;
            }

            result = result.max(seq_len);
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
    #[case(vec![100,4,200,1,3,2], 4)]
    #[case(vec![0,3,7,2,5,8,4,6,0,1], 9)]
    #[case(vec![1,0,1,2], 3)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::longest_consecutive(nums);
        assert_eq!(actual, expected);
    }
}
