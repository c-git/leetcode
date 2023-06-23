//! Solution for https://leetcode.com/problems/kids-with-the-greatest-number-of-candies
//! 1431. Kids With the Greatest Number of Candies

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = candies.iter().max().expect("By constraint");
        let threshold = max - extra_candies;
        candies.into_iter().map(|x| x >= threshold).collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,3,5,1,3], 3, vec![true,true,true,false,true] )]
    #[case(vec![4,2,1,1,2], 1, vec![true,false,false,false,false])]
    #[case(vec![12,1,12], 10, vec![true,false,true])]
    fn case(#[case] candies: Vec<i32>, #[case] extra_candies: i32, #[case] expected: Vec<bool>) {
        let actual = Solution::kids_with_candies(candies, extra_candies);
        assert_eq!(actual, expected);
    }
}
