//! Solution for https://leetcode.com/problems/sliding-window-median
//! 480. Sliding Window Median

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;
        let mut result = Vec::with_capacity(nums.len() - k + 1);

        // Establish preconditions for loop
        let mut sorted_window = Vec::with_capacity(k);
        for x in nums.iter().copied().take(k) {
            sorted_window.push(x);
        }
        sorted_window.sort_unstable();
        result.push(Self::median_of(&sorted_window));

        let mut pop_value = nums[0];
        for window in nums.windows(k).skip(1) {
            Self::remove_value(pop_value, &mut sorted_window);
            pop_value = window[0];
            Self::insert_value(*window.last().unwrap(), &mut sorted_window);
            result.push(Self::median_of(&sorted_window));
        }

        result
    }

    fn remove_value(pop_value: i32, sorted_window: &mut Vec<i32>) {
        let index = sorted_window.binary_search(&pop_value).unwrap();
        sorted_window.remove(index);
    }

    fn insert_value(new_value: i32, sorted_window: &mut Vec<i32>) {
        let index = match sorted_window.binary_search(&new_value) {
            Ok(idx) | Err(idx) => idx,
        };
        sorted_window.insert(index, new_value);
    }

    fn median_of(sorted_window: &[i32]) -> f64 {
        if sorted_window.len() % 2 == 0 {
            let mid = sorted_window.len() / 2;
            (sorted_window[mid] as f64 + sorted_window[mid - 1] as f64) / 2.0
        } else {
            let mid = sorted_window.len() / 2;
            sorted_window[mid] as f64
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
    #[case(vec![1,3,-1,-3,5,3,6,7], 3, vec![1.00000,-1.00000,-1.00000,3.00000,5.00000,6.00000])]
    #[case(vec![1,2,3,4,2,3,1,4,2], 3, vec![2.00000,3.00000,3.00000,3.00000,2.00000,3.00000,2.00000])]
    #[case(vec![1,3,-1,-3,5,3,6,7], 8, vec![3.0])]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: Vec<f64>) {
        let actual = Solution::median_sliding_window(nums, k);
        assert_eq!(actual, expected);
    }
}
