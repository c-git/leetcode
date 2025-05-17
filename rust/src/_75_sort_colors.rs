//! Solution for https://leetcode.com/problems/sort-colors
//! 75. Sort Colors

impl Solution {
    pub fn sort_colors(nums: &mut [i32]) {
        let freq_count = nums.iter().fold([0, 0, 0], |mut acc, &num| {
            acc[num as usize] += 1;
            acc
        });
        for num in nums.iter_mut().take(freq_count[0]) {
            *num = 0;
        }
        for num in nums.iter_mut().skip(freq_count[0]).take(freq_count[1]) {
            *num = 1;
        }
        for num in nums.iter_mut().skip(freq_count[0] + freq_count[1]) {
            *num = 2;
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
    #[case(vec![2,0,2,1,1,0])]
    #[case(vec![2,0,1])]
    fn case(#[case] mut nums: Vec<i32>) {
        let mut expected = nums.clone();
        expected.sort_unstable();
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, expected);
    }
}
