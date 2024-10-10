//! Solution for https://leetcode.com/problems/majority-element
//! 169. Majority Element

impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut result = *nums.first().expect("guaranteed to exists by constraints");
        let mut max_count = 0;
        let mut last = result;
        let mut curr_count = max_count;
        for num in nums {
            if last == num {
                curr_count += 1;
            } else {
                if max_count < curr_count {
                    max_count = curr_count;
                    result = last;
                }
                last = num;
                curr_count = 1;
            }
        }
        if max_count < curr_count {
            last
        } else {
            result
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
    #[case(vec![3,2,3], 3)]
    #[case(vec![2,2,1,1,1,2,2], 2)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::majority_element(nums);
        assert_eq!(actual, expected);
    }
}
