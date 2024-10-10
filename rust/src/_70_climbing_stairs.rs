//! Solution for https://leetcode.com/problems/climbing-stairs
//! 70. Climbing Stairs

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 1 {
            return 1;
        }
        let mut prev_prev = 1; // Step 0 (starting position one way to get here)
        let mut prev = 1; // Step 1 (only one way to get here take a step)
        for _ in 2..=n {
            // For each step you can take one step from prev so same number of steps as prev or
            // you can take 2 steps from the one before so same number as that
            // adding them together we get the way to get to the next step (Hey it's a Fibonacci sequence)
            let temp = prev + prev_prev;
            prev_prev = prev;
            prev = temp;
        }
        prev
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, 2)]
    #[case(3, 3)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::climb_stairs(n);
        assert_eq!(actual, expected);
    }
}
