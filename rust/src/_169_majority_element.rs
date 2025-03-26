//! Solution for https://leetcode.com/problems/majority-element
//! 169. Majority Element

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // Solved follow up by looking at solution from https://www.youtube.com/watch?v=ZQHruHVU3zM

        // Intuition: Because the majority must exist it will be more than half
        // of all the values and as a result if we just keep track of the net
        // amount of a number until it goes to 0 then we change to a new
        // candidate

        let mut candidate = *nums.first().expect("min length guaranteed to be 1");
        let mut net_count = 1usize;
        for num in nums.into_iter().skip(1) {
            let is_same = num == candidate;
            match (is_same, net_count) {
                (true, _) => net_count += 1,
                (false, 0) => {
                    candidate = num;
                    net_count = 1;
                }
                (false, _) => net_count -= 1,
            }
        }
        candidate
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,2,3], 3)]
    #[case(vec![2,2,1,1,1,2,2], 2)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::majority_element(nums);
        assert_eq!(actual, expected);
    }
}
