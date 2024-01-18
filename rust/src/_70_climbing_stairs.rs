//! Solution for https://leetcode.com/problems/climbing-stairs
//! 70. Climbing Stairs

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memo = vec![None; n as usize + 1];
        memo[0] = Some(1);
        memo[1] = Some(1);
        Self::climb_stairs_(n as usize, &mut memo)
    }

    fn climb_stairs_(n: usize, memo: &mut [Option<i32>]) -> i32 {
        debug_assert!(
            memo[0].is_some() && memo[1].is_some(),
            "Assumed to be set before first call"
        );
        if let Some(val) = memo[n] {
            return val;
        }

        // n is at least 2 because 0 and 1 come preset
        let result = Self::climb_stairs_(n - 1, memo) + Self::climb_stairs_(n - 2, memo);

        memo[n] = Some(result);
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
    #[case(2, 2)]
    #[case(3, 3)]
    #[case(45, 1_836_311_903)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::climb_stairs(n);
        assert_eq!(actual, expected);
    }
}
