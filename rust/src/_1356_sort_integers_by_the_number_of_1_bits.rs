//! Solution for https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits
//! 1356. Sort Integers by The Number of 1 Bits

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_by_cached_key(|x| (x.count_ones(), *x));
        arr
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![0,1,2,3,4,5,6,7,8], vec![0,1,2,4,8,3,5,6,7])]
    #[case(vec![1024,512,256,128,64,32,16,8,4,2,1], vec![1,2,4,8,16,32,64,128,256,512,1024])]
    fn case(#[case] arr: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::sort_by_bits(arr);
        assert_eq!(actual, expected);
    }
}
