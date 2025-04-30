//! Solution for https://leetcode.com/problems/perfect-squares
//! 279. Perfect Squares

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        // Find perfect squares in range
        let perfect_squares: Vec<i32> = (1..).map(|x| x * x).take_while(|&x| x <= n).collect();

        let mut dp = Vec::with_capacity(n as usize + 1); // Least number of steps to get the answer for each index

        dp.push(0); // For index 0 we need 0 steps to make a total of 0

        // Fill in dp from smallest to largest
        for num in 1..=n {
            let mut best = i32::MAX; // Stores best answer for `num`

            // Check each perfect square and see which produces the smallest number of steps when removed from `num`
            for perfect_square in perfect_squares.iter().take_while(|&&x| x <= num) {
                best = best.min(dp[(num - *perfect_square) as usize] + 1);
            }

            // Add best found to `dp`
            dp.push(best);
        }
        *dp.last().unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(12, 3)]
    #[case(13, 2)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::num_squares(n);
        assert_eq!(actual, expected);
    }
}
