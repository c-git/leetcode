//! Solution for https://leetcode.com/problems/largest-rectangle-in-histogram
//! 84. Largest Rectangle in Histogram

impl Solution {
    /// Based on https://www.youtube.com/watch?v=zx5Sw9130L0
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut result = 0;
        let n = heights.len();
        // Height and index pairs
        let mut smallest_prev = vec![(0, 0)]; // We can always get at least 0 at the start
        for (i, height) in heights.into_iter().enumerate() {
            let mut can_start_from = i;
            while height < smallest_prev.last().unwrap().0 {
                let (prev_height, prev_start_idx) = smallest_prev.pop().unwrap();
                can_start_from = prev_start_idx; // Was taller so we could have started there

                // Calculate area possible with bar popped
                result = result.max(prev_height * (i - prev_start_idx) as i32);
            }
            smallest_prev.push((height, can_start_from));
        }

        // Check values left on stack
        while let Some((prev_height, prev_start_idx)) = smallest_prev.pop() {
            // Calculate area possible with bar popped, given it goes to the end. If it
            // didn't then it would have been popped earlier
            result = result.max(prev_height * (n - prev_start_idx) as i32);
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
    #[case(vec![2,1,5,6,2,3], 10)]
    #[case(vec![2,4], 4)]
    fn case(#[case] heights: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::largest_rectangle_area(heights);
        assert_eq!(actual, expected);
    }
}
