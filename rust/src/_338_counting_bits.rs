//! Solution for https://leetcode.com/problems/counting-bits
//! 338. Counting Bits

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        (0..=n).map(|x| x.count_ones() as i32).collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, vec![0,1,1])]
    #[case(5, vec![0,1,1,2,1,2])]
    fn case(#[case] n: i32, #[case] expected: Vec<i32>) {
        let actual = Solution::count_bits(n);
        assert_eq!(actual, expected);
    }
}
