//! Solution for https://leetcode.com/problems/non-overlapping-intervals
//! 435. Non-overlapping Intervals

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable();

        // Each position stores the minimum number of intervals that must be removed to
        // include that element considering only the elements before it
        let mut dp = vec![usize::MAX; intervals.len()];
        dp[0] = 0; // We can always take the first one

        for (curr_idx, curr_interval) in intervals.iter().enumerate() {
            for (prev_idx, (take_cost, prev_interval)) in intervals
                .iter()
                .enumerate()
                .take(curr_idx)
                .rev()
                .enumerate()
            {
                let new_cost = if prev_interval[1] <= curr_interval[0] {
                    // Extendable so add the cost to extend using this index
                    dp[prev_idx] + take_cost
                } else {
                    // Skip all previous if we only consider this value as we cannot extend
                    curr_idx
                };
                dp[curr_idx] = dp[curr_idx].min(new_cost);
            }
        }

        // Answer is to take the lowest number from the right with a cost of +1 for each
        // element we go left as those to the right would have to be removed
        dp.into_iter()
            .rev()
            .enumerate()
            .fold(usize::MAX, |best: usize, (extra_cost, pos_cost)| {
                best.min(extra_cost + pos_cost)
            }) as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,2],vec![2,3],vec![3,4],vec![1,3]], 1)]
    #[case(vec![vec![1,2],vec![1,2],vec![1,2]], 2)]
    #[case(vec![vec![1,2],vec![2,3]], 0)]
    fn case(#[case] intervals: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::erase_overlap_intervals(intervals);
        assert_eq!(actual, expected);
    }
}
