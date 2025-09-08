//! Solution for https://leetcode.com/problems/3sum
//! 15. 3Sum

impl Solution {
    // Based on https://www.youtube.com/watch?v=wCe-MeqXgMc and previous solution
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        nums.sort_unstable();
        for (left_idx, left) in nums.iter().enumerate() {
            if left_idx > 0 && left == &nums[left_idx - 1] {
                // Skip duplicates
                continue;
            }
            let mut mid_idx = left_idx + 1;
            let mut right_idx = nums.len() - 1;
            while mid_idx < right_idx {
                let total = left + nums[mid_idx] + nums[right_idx];
                match total.cmp(&0) {
                    std::cmp::Ordering::Less => {
                        // Increase size as total is too small
                        mid_idx += 1;
                    }
                    std::cmp::Ordering::Equal => {
                        result.push(vec![*left, nums[mid_idx], nums[right_idx]]);
                        mid_idx += 1;
                        while mid_idx < right_idx && nums[mid_idx - 1] == nums[mid_idx] {
                            // Skip duplicates
                            mid_idx += 1;
                        }
                    }
                    std::cmp::Ordering::Greater => {
                        // Decrease size too big
                        right_idx -= 1;
                    }
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
