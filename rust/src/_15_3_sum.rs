//! Solution for https://leetcode.com/problems/3sum
//! 15. 3Sum

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        nums.sort_unstable();
        for left in 0..nums.len() {
            if left > 0 && nums[left] == nums[left - 1] {
                // Skip duplicates
                continue;
            }
            let mut middle = left + 1;
            let mut right = nums.len() - 1;
            while middle < right {
                let sum = nums[left] + nums[middle] + nums[right];
                match sum.cmp(&0) {
                    std::cmp::Ordering::Less => middle += 1,
                    std::cmp::Ordering::Equal => {
                        result.push(vec![nums[left], nums[middle], nums[right]]);
                        middle += 1;
                        while middle < right && nums[middle] == nums[middle - 1] {
                            // Skip duplicates
                            middle += 1;
                        }
                    }
                    std::cmp::Ordering::Greater => right -= 1,
                }
            }
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
    #[case(vec![-1,0,1,2,-1,-4], vec![vec![-1,-1,2],vec![-1,0,1]])]
    #[case(vec![0,1,1], vec![])]
    #[case(vec![0,0,0], vec![vec![0,0,0]])]
    #[case(vec![0,0,0,0], vec![vec![0,0,0]])]
    fn case(#[case] nums: Vec<i32>, #[case] mut expected: Vec<Vec<i32>>) {
        let mut actual = Solution::three_sum(nums);
        actual.iter_mut().for_each(|x| x.sort_unstable());
        actual.sort_unstable();
        expected.iter_mut().for_each(|x| x.sort_unstable());
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }
}
