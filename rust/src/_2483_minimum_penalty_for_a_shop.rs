//! Solution for https://leetcode.com/problems/minimum-penalty-for-a-shop
//! 2483. Minimum Penalty for a Shop

impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        // After reading editorial

        // Start with closing at hour 0 and assume the current penalty is 0.
        let mut cur_penalty = 0;
        let mut min_penalty = 0;
        let mut result_earliest_hour = 0;

        for (i, c) in customers.as_bytes().iter().enumerate() {
            // If status in hour i is 'Y', moving it to open hours decrement
            // penalty by 1. Otherwise, moving 'N' to open hours increment
            // penalty by 1.

            if c == &b'Y' {
                cur_penalty -= 1;
            } else {
                cur_penalty += 1;
            }

            // Update earliest_hour if a smaller penalty is encountered
            if cur_penalty < min_penalty {
                result_earliest_hour = i + 1;
                min_penalty = cur_penalty;
            }
        }

        //     return earliest_hour
        result_earliest_hour as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("YYNY", 2)]
    #[case("NNNNN", 0)]
    #[case("YYYY", 4)]
    #[case("N", 0)]
    #[case("Y", 1)]
    fn case(#[case] customers: String, #[case] expected: i32) {
        let actual = Solution::best_closing_time(customers);
        assert_eq!(actual, expected);
    }
}
