//! Solution for https://leetcode.com/problems/largest-divisible-subset
//! 368. Largest Divisible Subset

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let mut largest_set_index_start = 0;
        let mut largest_set_size = 1;
        for (i, mut num) in nums.iter().copied().enumerate() {
            let size = nums
                .iter()
                .skip(i)
                .filter(|&&x| {
                    if x % num == 0 {
                        num = x;
                        true
                    } else {
                        false
                    }
                })
                .count();
            if size > largest_set_size {
                largest_set_index_start = i;
                largest_set_size = size;
            }
        }
        let mut curr_value = nums[largest_set_index_start];
        nums.retain(|&num| {
            if num % curr_value == 0 {
                curr_value = num;
                true
            } else {
                false
            }
        });
        nums
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3], vec![1,2])]
    #[case(vec![1,2,4,8], vec![1,2,4,8])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::largest_divisible_subset(nums);
        assert_eq!(actual, expected);
    }
}
