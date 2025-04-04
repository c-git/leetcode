//! Solution for https://leetcode.com/problems/single-number-iii
//! 260. Single Number III

impl Solution {
    /// Based on https://www.youtube.com/watch?v=faoVORjd-T8
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        // Intuition: If we xor all the numbers then the bits still set at the end were
        // in one of the single numbers only otherwise there would be an even number of
        // them and they would have xor'ed to 0. So if we separate all the number into
        // two groups those with that number and those without then our two single
        // number will be in different groups and thus each pair will cancel out each
        // other except our single numbers
        
        let xor_all = nums.iter().copied().fold(0, |acc, x| acc ^ x);
        let diff_bit_pos = xor_all.trailing_zeros() + 1;
        let diff_mask = 1 << diff_bit_pos;
        let answer1 = nums.iter().copied().fold(0, |acc, x| if x & diff_mask == 0 { acc ^ x} else {acc});
        let answer2 = nums.iter().copied().fold(0, |acc, x| if x & diff_mask != 0 { acc ^ x} else {acc});
        vec![answer1, answer2]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,1,3,2,5], vec![3,5])]
    #[case(vec![-1,0], vec![-1,0])]
    #[case(vec![0,1], vec![1,0])]
    fn case(#[case] nums: Vec<i32>, #[case] mut expected: Vec<i32>) {
        let mut actual = Solution::single_number(nums);
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
}
