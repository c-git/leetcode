use std::{
    cmp::{max, min, Ordering},
    collections::HashMap,
};

impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        // Calculate frequency of sums
        let mut frequencies: HashMap<i32, i32> = HashMap::new();
        let n = nums.len();
        for i in 0..n / 2 {
            let pair_sum = nums[i] + nums[n - 1 - i];
            *frequencies.entry(pair_sum).or_default() += 1;
        }

        if frequencies.len() == 1 {
            return 0; // Only one sum
        }

        // Create vec of most common pair_sums
        let mut most_popular_pair_sums = Vec::new();
        let mut max_freq = 0;
        for (pair_sum, frequency) in frequencies {
            match frequency.cmp(&max_freq) {
                Ordering::Less => {} // Just ignore this cannot be part of most popular
                Ordering::Equal => most_popular_pair_sums.push(pair_sum),
                Ordering::Greater => {
                    most_popular_pair_sums.clear();
                    most_popular_pair_sums.push(pair_sum);
                    max_freq = frequency;
                }
            }
        }

        // Try all popular sums to see which produces the least required changes
        let mut min_moves = i32::MAX;
        for target_sum in most_popular_pair_sums {
            let mut moves = 0;
            for i in 0..n / 2 {
                let curr_sum = nums[i] + nums[n - 1 - i];
                let diff = target_sum - curr_sum;
                match diff.cmp(&0) {
                    Ordering::Less => {
                        // Diff is negative see if we can reduce the larger number enough to hit the target otherwise we need to change both
                        let larger = max(nums[i], nums[n - 1 - i]);
                        moves += if larger + diff <= 0 { 2 } else { 1 };
                    }
                    Ordering::Equal => {} // Do nothing already on target
                    Ordering::Greater => {
                        // Diff is positive see if we can increase the smaller number enough to hit the target otherwise we need to change both
                        let smaller = min(nums[i], nums[n - 1 - i]);
                        moves += if smaller + diff > limit { 2 } else { 1 };
                    }
                }
            }
            min_moves = min(min_moves, moves);
            if min_moves == 1 {
                // No need to check more this is the best we can do
                return 1;
            }
        }
        min_moves
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,4,3], 4, 1)]
    #[case(vec![1,2,2,1], 2, 2)]
    #[case(vec![1,2,1,2], 2, 0)]
    #[case(vec![1,1,2,2,1,1], 2, 2)]
    fn case(#[case] nums: Vec<i32>, #[case] limit: i32, #[case] expected: i32) {
        let actual = Solution::min_moves(nums, limit);
        assert_eq!(actual, expected);
    }
}
