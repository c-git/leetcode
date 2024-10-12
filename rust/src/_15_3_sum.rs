//! Solution for https://leetcode.com/problems/3sum
//! 15. 3Sum

use std::collections::BTreeSet;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut result: BTreeSet<Vec<i32>> = BTreeSet::new();
        for first in 0..nums.len() - 2 {
            if nums[first] > 0 {
                break;
            }
            if first > 0 && nums[first - 1] == nums[first] {
                continue;
            }
            for second in first + 1..nums.len() - 1 {
                let target = 0 - nums[first] - nums[second];
                if nums[second + 1..].binary_search(&target).is_ok() {
                    result.insert(vec![nums[first], nums[second], target]);
                }
            }
        }
        result.into_iter().collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![-1,0,1,2,-1,-4], vec![vec![-1,-1,2],vec![-1,0,1]])]
    #[case(vec![0,1,1], vec![])]
    #[case(vec![0,0,0], vec![vec![0,0,0]])]
    #[case(vec![0,0,0,0], vec![vec![0,0,0]])]
    #[case(vec![-1,0,1,2,-1,-4,-2,-3,3,0,4], vec![vec![-4,0,4],vec![-4,1,3],vec![-3,-1,4],vec![-3,0,3],vec![-3,1,2],vec![-2,-1,3],vec![-2,0,2],vec![-1,-1,2],vec![-1,0,1]])]
    #[case(vec![-4,-2,-2,-2,0,1,2,2,2,3,3,4,4,6,6], vec![vec![-4,-2,6],vec![-4,0,4],vec![-4,1,3],vec![-4,2,2],vec![-2,-2,4],vec![-2,0,2]])]
    fn case(#[case] nums: Vec<i32>, #[case] mut expected: Vec<Vec<i32>>) {
        let mut actual = Solution::three_sum(nums);
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
}
